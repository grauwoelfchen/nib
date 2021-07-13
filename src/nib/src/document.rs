use document_tree::element_categories::{
    StructuralSubElement as SSE, TextOrInlineElement as TOIE,
};
use document_tree::elements::CommonAttributes;
use document_tree::extra_attributes;

pub struct MyDocument {
    pub children: Vec<SSE>,
}

impl MyDocument {
    pub fn children(&self) -> Vec<SSE> {
        self.children.clone()
    }
}

#[allow(dead_code)]
pub struct MySection {
    common: CommonAttributes,
    pub children: Vec<SSE>,
}

impl MySection {
    pub fn children(&self) -> Vec<SSE> {
        self.children.clone()
    }
}

#[allow(dead_code)]
pub struct MyCommonAttributes {
    ids: Vec<String>,
    names: Vec<String>,
    source: Option<String>,
    pub classes: Vec<String>,
}

#[allow(dead_code)]
pub struct MyLiteralBlock {
    pub common: CommonAttributes,
    extra: extra_attributes::LiteralBlock,
    pub children: Vec<TOIE>,
}

impl MyLiteralBlock {
    pub fn children(&self) -> Vec<TOIE> {
        self.children.clone()
    }
}

#[allow(dead_code)]
pub fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
