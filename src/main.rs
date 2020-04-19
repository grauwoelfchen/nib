//! Beta
//!
//! # Examples
//!
//! ```zsh
//! % beta
//! ```
use std::fs;
use std::result::Result;

use beta::{generate, get_entries, skip_meta};

fn main() {
    let ptrn = "blog/*.rst";
    for e in get_entries(ptrn).filter_map(Result::ok) {
        let s = fs::read_to_string(e).unwrap();
        let r = generate(&format!("{}\n", skip_meta(&s)));
        println!("{}\n", r.unwrap());
    }
}
