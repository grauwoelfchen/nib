use std::io::{Error, ErrorKind};
use std::path::Path;

use handlebars::{Handlebars, no_escape};

const TMPL: [&str; 8] = [
    "_article",
    "_footer",
    "_header",
    "_sidebar",
    "headline",
    "index",
    "layout",
    "layout.idx",
];

pub fn init_registry<'a>() -> Result<Handlebars<'a>, Error> {
    let mut reg = Handlebars::new();
    // get this file's directory
    let dir = Path::new(file!()).parent().expect("can't get a directory");
    let thm = Path::new(dir).join("theme");

    for n in TMPL.iter() {
        reg.register_template_file(n, thm.join(format!("{}.hbs", n)))
            .as_ref()
            .map_err(|e| {
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
