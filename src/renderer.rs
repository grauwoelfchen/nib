//!
use std::io::{BufWriter, Error, ErrorKind};

use rst_parser::parse;
use rst_renderer::render_html;
use document_tree::Document;
use document_tree::element_categories::{
    BodyElement, StructuralSubElement, SubStructure, TextOrInlineElement,
};
use document_tree::elements::{CommonAttributes, LiteralBlock, Section};
use document_tree::extra_attributes;

/// returns HTML result rendered in partial mode.
pub fn render(s: &str) -> Result<String, Error> {
    match parse(s) {
        Err(e) => {
            eprintln!("err: {}", e);
            Err(Error::new(ErrorKind::InvalidInput, e))
        },
        Ok(mut doc) => render_html_with_decoration(&mut doc),
    }
}

struct MyDocument {
    pub children: Vec<StructuralSubElement>,
}

impl MyDocument {
    pub fn children(&self) -> Vec<StructuralSubElement> {
        self.children.clone()
    }
}

#[allow(dead_code)]
struct MySection {
    common: CommonAttributes,
    pub children: Vec<StructuralSubElement>,
}

impl MySection {
    pub fn children(&self) -> Vec<StructuralSubElement> {
        self.children.clone()
    }
}

#[allow(dead_code)]
struct MyLiteralBlock {
    common: CommonAttributes,
    extra: extra_attributes::LiteralBlock,
    pub children: Vec<TextOrInlineElement>,
}

impl MyLiteralBlock {
    pub fn children(&self) -> Vec<TextOrInlineElement> {
        self.children.clone()
    }
}

#[allow(dead_code)]
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn highlight(b: &LiteralBlock) {
    let mb: &mut MyLiteralBlock = unsafe {
        #[allow(mutable_transmutes)]
        #[allow(clippy::transmute_ptr_to_ptr)]
        std::mem::transmute(&*b)
    };
    for (k, t) in mb.children().iter().enumerate() {
        match t {
            TextOrInlineElement::String(ref v) => {
                // TODO: syntect
                // println!("{}", v);
                let b = Box::new((**v).to_string());
                mb.children[k] = TextOrInlineElement::String(b);
            },
            _ => continue,
        }
    }
}

fn render_html_with_decoration(doc: &mut Document) -> Result<String, Error> {
    let standalone = false;
    let buf = Vec::new();
    let mut stream = BufWriter::new(buf);

    let md: &mut MyDocument =
        unsafe { &mut *(doc as *mut Document as *mut MyDocument) };

    for (i, elm) in md.children().iter().enumerate() {
        match elm {
            StructuralSubElement::SubStructure(ref s1) => {
                match **s1 {
                    SubStructure::Section(ref s) => {
                        let ms: &mut MySection = unsafe {
                            #[allow(clippy::cast_ref_to_mut)]
                            &mut *(&**s as *const Section as *mut MySection)
                        };
                        for (j, se) in ms.children().iter().enumerate() {
                            match se {
                                StructuralSubElement::SubStructure(ref s2) => {
                                    match **s2 {
                                        SubStructure::BodyElement(ref e) => {
                                            match **e {
                                                BodyElement::LiteralBlock(
                                                    ref b,
                                                ) => highlight(b),
                                                _ => continue,
                                            }
                                        },
                                        _ => continue,
                                    }
                                },
                                _ => continue,
                            }
                            ms.children[j] = (*se).clone();
                        }
                    },
                    SubStructure::BodyElement(ref e) => {
                        match **e {
                            BodyElement::LiteralBlock(ref b) => highlight(b),
                            _ => continue,
                        }
                    },
                    _ => continue,
                }
            },
            _ => continue,
        }
        md.children[i] = (*elm).clone();
    }

    render_html(&doc, &mut stream, standalone)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let r = stream.into_inner().unwrap();
    Ok(String::from_utf8_lossy(&r).into_owned())
}
