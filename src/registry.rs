use std::io::{Error, ErrorKind};
use std::path::Path;

use handlebars::{Handlebars, no_escape};

pub fn load_registry<'a>() -> Result<Handlebars<'a>, Error> {
    let mut reg = Handlebars::new();
    // get this file's directory
    let dir = Path::new(file!()).parent().expect("can't get a directory");
    let tmpl = Path::new(dir).join("tmpl");

    for r in [
        reg.register_template_file("_article", tmpl.join("_article.hbs")),
        reg.register_template_file("_footer", tmpl.join("_footer.hbs")),
        reg.register_template_file("_header", tmpl.join("_header.hbs")),
        reg.register_template_file("_headline", tmpl.join("_headline.hbs")),
        reg.register_template_file("_index", tmpl.join("_index.hbs")),
        reg.register_template_file("_layout", tmpl.join("_layout.hbs")),
        reg.register_template_file("_layout.idx", tmpl.join("_layout.idx.hbs")),
    ]
    .iter()
    {
        r.as_ref().map_err(|e| {
            eprintln!("err: {}", e);
            Error::new(ErrorKind::InvalidInput, "no such template file")
        })?;
    }
    Ok(reg)
}

pub fn add_escape_fn(reg: &mut Handlebars) {
    reg.register_escape_fn(no_escape)
}

pub fn rem_escape_fn(reg: &mut Handlebars) {
    reg.unregister_escape_fn()
}
