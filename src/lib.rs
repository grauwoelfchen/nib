use std::io::{BufWriter, Error, ErrorKind};

use glob::glob;
use rst_parser::parse;
use rst_renderer::render_html;

/// gets file entries.
pub fn get_entries(path: &str) -> glob::Paths {
    glob(path).expect("failed to read glob pattern")
}

/// skips metadata at the beginnig of each file.
pub fn skip_meta(s: &str) -> String {
    let mut v = s.lines();
    loop {
        match v.next() {
            None => break,
            Some(a) if !a.starts_with(".. ") => break,
            _ => (),
        }
    }
    let a: Vec<&str> = v.collect();
    a.join("\n")
}

/// renders HTML result in partial mode.
pub fn generate(s: &str) -> Result<String, Error> {
    match parse(s) {
        Err(e) => {
            eprintln!("err: {}", e);
            Err(Error::new(ErrorKind::InvalidInput, e))
        },
        Ok(doc) => {
            let buf = Vec::new();
            let mut stream = BufWriter::new(buf);
            let standalone = false;

            render_html(&doc, &mut stream, standalone)
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
            let r = stream.into_inner().unwrap();
            Ok(String::from_utf8_lossy(&r).into_owned())
        },
    }
}
