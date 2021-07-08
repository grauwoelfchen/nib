use std::io::{Error, ErrorKind};

use handlebars::{Handlebars, no_escape};

use crate::include_template_file;

pub fn init_registry<'a>() -> Result<Handlebars<'a>, Error> {
    let mut reg = Handlebars::new();

    let theme = "documentation";

    // TODO: support user defined template
    #[allow(clippy::vec_init_then_push)]
    for (n, s) in match theme {
        "documentation" => include_template_file!(
            "documentation",
            "_article",
            "_sidebar",
            "layout"
        ),
        _ => include_template_file!(
            "blog", "_article", "_footer", "_header", "_sidebar", "headline",
            "layout"
        ),
    } {
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
