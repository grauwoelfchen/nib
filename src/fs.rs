//!
#[cfg(not(test))]
use std::fs;
use std::io::Error;

use glob::glob;

/// fetches given entries.
pub fn get_entries(path: &str) -> glob::Paths {
    glob(path).expect("failed to read glob pattern")
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

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use super::*;

    pub fn dummy_remove_file(_: PathBuf) -> std::io::Result<()> {
        Ok(())
    }

    #[test]
    fn test_get_entries_with_single_file_pattern() {
        let path = file!();

        let buf = get_entries(path).next().expect("next").unwrap();
        assert_eq!(Path::new("src/fs.rs"), buf.as_path());
    }

    #[test]
    fn test_get_entries_with_glob_pattern() {
        let ptrn = Path::new(file!()).parent().unwrap().join("fs.*");
        let path = ptrn.as_os_str().to_str().unwrap();

        let buf = get_entries(path).next().expect("next").unwrap();
        assert_eq!(Path::new("src/fs.rs"), buf.as_path());
    }

    #[test]
    fn test_rem_results() {
        let path = file!();
        assert!(rem_results(path).is_ok());
    }
}
