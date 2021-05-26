//!
use std::io::{BufWriter, Error, ErrorKind};

use rst_parser::parse;
use rst_renderer::render_html;
use document_tree::Document;
use document_tree::attribute_types::FixedSpace;
use document_tree::element_categories::{
    BodyElement as BE, StructuralSubElement as SSE, SubStructure as SS,
    TextOrInlineElement as TOIE,
};
use document_tree::elements::{CommonAttributes, LiteralBlock, Raw, Section};
use document_tree::extra_attributes;

use crate::highlighter::Highlighter;

/// returns HTML result rendered in partial mode.
pub fn render(s: &str) -> Result<String, Error> {
    match parse(s) {
        Err(e) => {
            eprintln!("err: {}", e);
            Err(Error::new(ErrorKind::InvalidInput, e))
        }
        Ok(mut doc) => render_html_with_highlight(&mut doc),
    }
}

struct MyDocument {
    pub children: Vec<SSE>,
}

impl MyDocument {
    pub fn children(&self) -> Vec<SSE> {
        self.children.clone()
    }
}

#[allow(dead_code)]
struct MySection {
    common: CommonAttributes,
    pub children: Vec<SSE>,
}

impl MySection {
    pub fn children(&self) -> Vec<SSE> {
        self.children.clone()
    }
}

#[allow(dead_code)]
struct MyCommonAttributes {
    ids: Vec<String>,
    names: Vec<String>,
    source: Option<String>,
    pub classes: Vec<String>,
}

#[allow(dead_code)]
struct MyLiteralBlock {
    common: CommonAttributes,
    extra: extra_attributes::LiteralBlock,
    pub children: Vec<TOIE>,
}

impl MyLiteralBlock {
    pub fn children(&self) -> Vec<TOIE> {
        self.children.clone()
    }
}

#[allow(dead_code)]
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn highlight(lb: &LiteralBlock, hi: &Highlighter) -> Raw {
    let mb: &mut MyLiteralBlock = unsafe {
        #[allow(mutable_transmutes)]
        #[allow(clippy::transmute_ptr_to_ptr)]
        std::mem::transmute(&*lb)
    };
    let mc: &mut MyCommonAttributes = unsafe {
        #[allow(mutable_transmutes)]
        #[allow(clippy::transmute_ptr_to_ptr)]
        std::mem::transmute(&mb.common)
    };

    let default_ext = &"txt".to_string();
    let ext = mc.classes.first().unwrap_or_else(|| default_ext);

    let mut txt = "".to_string();
    for t in mb.children() {
        if let TOIE::String(ref v) = t {
            txt.push_str(&**v);
        }
    }

    // LiteralBlock -> Raw (pre)
    Raw::new(
        mb.common.clone(),
        extra_attributes::Raw {
            space: FixedSpace::default(),
            format: vec![],
        },
        vec![hi.apply(txt, ext)],
    )
}

fn hightlight_doc(doc: &mut Document) {
    let hi = Highlighter::new();
    let md: &mut MyDocument =
        unsafe { &mut *(doc as *mut Document as *mut MyDocument) };

    for (i, e) in md.children().iter().enumerate() {
        if let SSE::SubStructure(ref s1) = e {
            match **s1 {
                SS::Section(ref s) => {
                    let ms: &mut MySection = unsafe {
                        #[allow(clippy::cast_ref_to_mut)]
                        &mut *(&**s as *const Section as *mut MySection)
                    };
                    for (j, se) in ms.children().iter().enumerate() {
                        if let SSE::SubStructure(ref s2) = se {
                            if let SS::BodyElement(ref be) = **s2 {
                                if let BE::LiteralBlock(ref lb) = **be {
                                    let rw = highlight(lb, &hi);
                                    let nb = SS::BodyElement(Box::new(
                                        BE::Raw(Box::new(rw)),
                                    ));
                                    ms.children[j] =
                                        SSE::SubStructure(Box::new(nb));
                                }
                            }
                        }
                    }
                    md.children[i] = (*e).clone();
                }
                SS::BodyElement(ref be) => {
                    if let BE::LiteralBlock(ref lb) = **be {
                        let rw = highlight(lb, &hi);
                        let nb =
                            SS::BodyElement(Box::new(BE::Raw(Box::new(rw))));
                        md.children[i] = SSE::SubStructure(Box::new(nb));
                    }
                }
                _ => (),
            }
        }
    }
}

fn render_html_with_highlight(doc: &mut Document) -> Result<String, Error> {
    let standalone = false;
    let buf = Vec::new();
    let mut stream = BufWriter::new(buf);

    hightlight_doc(doc);

    render_html(&doc, &mut stream, standalone)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let r = stream.into_inner().unwrap();
    Ok(String::from_utf8_lossy(&r).into_owned())
}
