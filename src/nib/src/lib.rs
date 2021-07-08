extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod metadata;
mod renderer;

pub mod config;
pub mod fs;
pub mod highlighter;
pub mod loader;
pub mod registry;
pub mod writer;

/// A macro loads handlebar files in default template with include_str! macro.
///
/// # Examples
///
/// ```rust
/// # use std::any::type_name;
/// #
/// # #[macro_use]
/// # use nib;
/// #
/// # fn get_type<T>(_: &T) -> &str {
/// #     type_name::<T>()
/// # }
/// // read file(s) from src/theme/blog/ dir
/// let v = nib::include_template_file!("blog", "layout");
///
/// assert_eq!(v.len(), 1);
/// assert_eq!(get_type(&v), "alloc::vec::Vec<(&str, &str)>");
///
/// let key = v[0].0;
/// let dat = v[0].1;
///
/// assert_eq!(key, "layout");
/// assert!(dat.starts_with("<!DOCTYPE html>\n"));
/// ```
#[macro_export]
macro_rules! include_template_file {
    ( $n:expr, $( $x:expr ),* ) => {
        {
            let mut t = Vec::new();
            $(
                // TODO: is it any way can omit this slash?
                t.push((
                    $x,
                    include_str!(concat!("theme/", $n, "/", $x, ".hbs"))
                ));
            )*
            t
        }
    };
}

/// A macro loads static files in default template with include_str! macro.
///
/// # Examples
///
/// ```rust
/// # use std::any::type_name;
/// #
/// # #[macro_use]
/// # use nib;
/// #
/// # fn get_type<T>(_: &T) -> &str {
/// #     type_name::<T>()
/// # }
/// // read file(s) from src/theme/blog/ dir
/// let v = nib::include_static_file!("blog", "css/index.css");
///
/// assert_eq!(v.len(), 1);
/// assert_eq!(get_type(&v), "alloc::vec::Vec<(&str, &str)>");
///
/// let key = v[0].0;
/// let dat = v[0].1;
///
/// dbg!(&dat);
/// assert_eq!(key, "css/index.css");
/// assert!(dat.starts_with("* {\n"));
/// ```
#[macro_export]
macro_rules! include_static_file {
    ( $n:expr, $( $x:expr ),* ) => {
        {
            let mut s = Vec::new();
            $(
                // TODO: is it any way can omit these slashes?
                s.push((
                    $x,
                    include_str!(concat!("theme/", $n, "/static/", $x)),
                ));
            )*
            s
        }
    };
}
