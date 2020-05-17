extern crate serde_derive;

#[macro_use]
extern crate serde_json;

mod metadata;
mod renderer;

pub mod config;
pub mod fs;
pub mod loader;
pub mod registry;
pub mod writer;

#[macro_export]
/// A macro loads handlebar files in default template with include_str! macro.
macro_rules! include_template_file(
    { $( $x:expr ),* } => {
        {
            let mut t = Vec::new();
            $(
                // TODO: is it any way can omit this slash?
                t.push(($x, include_str!(concat!("theme/", $x, ".hbs"))));
            )*
            t
        }
    };
);

#[macro_export]
/// A macro loads static files in default template with include_str! macro.
macro_rules! include_static_file(
    { $( $x:expr ),* } => {
        {
            let mut s = Vec::new();
            $(
                // TODO: is it any way can omit these slashes?
                s.push((
                    $x,
                    include_str!(concat!("theme/static/", $x)),
                ));
            )*
            s
        }
    };
);
