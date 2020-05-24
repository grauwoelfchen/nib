use std::io::{Error, ErrorKind};

use handlebars::{Handlebars, no_escape};

use crate::include_template_file;

pub fn init_registry<'a>() -> Result<Handlebars<'a>, Error> {
    let mut reg = Handlebars::new();

    // TODO: support user defined template
    for (n, s) in include_template_file!(
        "_article", "_footer", "_header", "_sidebar", "headline", "layout"
    ) {
        reg.register_template_string(n, s).as_ref().map_err(|e| {
            eprintln!("err: {}", e);
            Error::new(ErrorKind::InvalidInput, "no such template file")
        })?;
    }
    Ok(reg)
}

pub fn add_escape_fn(reg: &mut Handlebars) {
    reg.register_escape_fn(no_escape)
}

pub fn del_escape_fn(reg: &mut Handlebars) {
    reg.unregister_escape_fn()
}
