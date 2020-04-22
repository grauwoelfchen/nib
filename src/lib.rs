use std::fs;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

use glob::glob;
use rst_parser::parse;
use rst_renderer::render_html;

/// skips metadata at the beginnig of each file.
fn skip_meta(s: &str) -> String {
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
fn render(s: &str) -> Result<String, Error> {
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

/// generates a HTML file into dst directory.
pub fn generate_entry(e: &PathBuf, dst: &str) -> Result<(), Error> {
    let stem = e.file_stem().unwrap().to_string_lossy().into_owned();
    let name = vec![stem, "html".to_string()].join(".");

    let s = fs::read_to_string(e)?;
    let r = render(&format!("{}\n", skip_meta(&s)));

    let path = Path::new(dst).join(name);
    let mut file = fs::File::create(path)?;

    // TODO: template
    file.write_all(r.unwrap().as_bytes())?;
    Ok(())
}

/// gets file entries.
pub fn get_entries(path: &str) -> glob::Paths {
    glob(path).expect("failed to read glob pattern")
}
