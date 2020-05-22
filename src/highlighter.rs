use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;

#[derive(Default)]
pub struct Highlighter {
    ss: SyntaxSet,
    ts: ThemeSet,
}

const THEME: &str = "base16-eighties.dark";

impl Highlighter {
    pub fn new() -> Self {
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        dbg!(&ts.themes);
        Self { ss, ts }
    }

    pub fn apply(&self, blk: String, ext: &str) -> String {
        let syn = self.ss.find_syntax_by_extension(ext);
        if syn.is_none() {
            return blk;
        }
        highlighted_html_for_string(
            &blk,
            &self.ss,
            &syn.unwrap(),
            &self.ts.themes[THEME],
        )
    }
}
