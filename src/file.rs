use glob::glob;

/// gets file entries.
pub fn get_entries(path: &str) -> glob::Paths {
    glob(path).expect("failed to read glob pattern")
}
