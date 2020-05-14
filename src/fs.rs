//!
#[cfg(not(test))]
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use glob::glob;

/// fetch given entries.
pub fn get_entries(paths: Vec<String>) -> Vec<PathBuf> {
    let mut tmp: Vec<glob::Paths> = vec![];
    for path in paths {
        tmp.push(glob(&path).expect("failed to read path pattern"));
    }
    let mut buf: Vec<PathBuf> = vec![];
    for t in tmp {
        for e in t.filter_map(Result::ok) {
            if !e.is_absolute() {
                buf.push(e);
            }
        }
    }
    buf
}

/// clear generated files.
pub fn rem_results(path: &str) -> Result<(), Error> {
    let results = glob(path).expect("failed to read glob pattern");
    for f in results.filter_map(std::result::Result::ok) {
        #[cfg(test)]
        test::dummy_remove_file(f)?;
        #[cfg(not(test))]
        fs::remove_file(f)?;
    }
    Ok(())
}

// TODO: Name
/// return string path which is ommited the most parent directory.
pub fn to_child_str_path(path: &Path) -> String {
    let mut prts = path.components();

    // skip the most parent directory
    prts.next();

    // build pathbuf from components
    let _path = prts.fold(PathBuf::new(), |mut acc, p| {
        acc.push(p);
        acc
    });
    _path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use super::*;

    pub fn dummy_remove_file(_: PathBuf) -> std::io::Result<()> {
        Ok(())
    }

    #[test]
    fn test_get_entries_with_single_file_pattern() {
        let src = vec![file!().to_string()];
        let ret = get_entries(src);
        let buf = ret.iter().next().expect("next");
        assert_eq!(Path::new("src/fs.rs"), buf.as_path());
    }

    #[test]
    fn test_get_entries_with_glob_pattern() {
        let path = Path::new(file!()).parent().unwrap().join("fs.*");

        let src = vec![path.as_os_str().to_string_lossy().into_owned()];
        let ret = get_entries(src);
        let buf = ret.iter().next().expect("next");
        assert_eq!(Path::new("src/fs.rs"), buf.as_path());
    }

    #[test]
    fn test_rem_results() {
        let path = file!();
        assert!(rem_results(path).is_ok());
    }

    #[test]
    fn test_to_child_str_path() {
        let path = Path::new("foo/bar/baz");
        assert_eq!(to_child_str_path(&path), "bar/baz");

        let path = Path::new("foo/bar/baz/qux");
        assert_eq!(to_child_str_path(&path), "bar/baz/qux");

        let path = Path::new("/foo/bar/baz/qux");
        assert_eq!(to_child_str_path(&path), "foo/bar/baz/qux");
    }
}
