//!
use std::io::{BufWriter, Error, ErrorKind};

use rst_parser::parse;
use rst_renderer::render_html;

/// returns HTML result rendered in partial mode.
pub fn render(s: &str) -> Result<String, Error> {
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
