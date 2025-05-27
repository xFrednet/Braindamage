use std::borrow::Cow;

use regex::{Captures, Regex, Replacer};

pub fn run_passes(input: &str) -> String {
    let pass = RepeatSymbol::default();
    if let Some(res) = pass.pass(input) {
        res
    } else {
        input.to_string()
    }
}

pub trait TextPass {
    fn pass(&self, input: &str) -> Option<String>;
}

#[derive(Debug)]
struct RepeatSymbol {
    re: Regex
}

impl Default for RepeatSymbol {
    fn default() -> Self {
        Self { re: Regex::new(r"REPEAT\((?<sym>.),(?<ctn>\d+)\)").unwrap() }
    }
}

impl TextPass for RepeatSymbol {
    fn pass(&self, input: &str) -> Option<String> {
        let res = self.re.replace_all(input, RepeatSymbolReplacer);
        match res {
            Cow::Owned(res) => Some(res),
            Cow::Borrowed(_) => None,
        }
    }
}

struct RepeatSymbolReplacer;
impl Replacer for RepeatSymbolReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        let loops: u32 = caps["ctn"].parse().unwrap();
        for _ in 0..loops {
            dst.push_str(&caps["sym"]);
        }
    }
}