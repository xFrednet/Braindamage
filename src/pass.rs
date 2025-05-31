use std::borrow::Cow;

use regex::{Regex, Replacer};

const PASS_LIMIT: u32 = 100;
const CHUNK_SIZE: u32 = 4;
const CHUNK_USE_OFFSET: u32 = 1;
const CHUNK_TEMP_OFFSET: u32 = 2;
const CHUNK_DATA_OFFSET: u32 = 3;

macro_rules! declare_regex_pass {
    ($name:ident, $regex:literal) => {
        struct $name {
            regex: Regex,
        }
        impl $name {
            fn new() -> Self {
                Self {
                    regex: Regex::new($regex).unwrap(),
                }
            }

            fn run<'a>(&self, src: &'a str) -> Cow<'a, str> {
                self.regex.replace_all::<&Self>(src, self)
            }
        }
    };
}

declare_regex_pass!(RepeatMacro, r"Repeat\((?<sym>[^,\s]+),\s*(?<ctn>\d+)\)");
impl Replacer for &RepeatMacro {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let sym = &caps["sym"];
        let n = caps["ctn"].parse::<u32>().unwrap();
        for _ in 0..n {
            dst.push_str(sym);
        }
    }
}

declare_regex_pass!(
    ChunkMacro,
    r"(?x)
        ((?<macro0>Frame|L|R)\((?<arg>\d+)\))
        | ((?<macro1>Use|Temp|Data)\((?<sym>[^\)]+)\))
    "
);
impl Replacer for &ChunkMacro {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        if let Some(mac) = caps.name("macro1") {
            let sym = &caps["sym"];
            let addition = match mac.as_str() {
                "Use" => format!("Repeat(>,{CHUNK_USE_OFFSET}){sym}Repeat(<,{CHUNK_USE_OFFSET})"),
                "Temp" => format!("Repeat(>,{CHUNK_TEMP_OFFSET}){sym}Repeat(<,{CHUNK_TEMP_OFFSET})"),
                "Data" => format!("Repeat(>,{CHUNK_DATA_OFFSET}){sym}Repeat(<,{CHUNK_DATA_OFFSET})"),
                _ => unreachable!("Unexpected symbol `{mac:?}({sym})`"),
            };
            dst.push_str(&addition);
        } else if let Some(mac) = caps.name("macro0") {
            let arg = caps["arg"].parse::<u32>().unwrap();
            let addition = match mac.as_str() {
                "Frame" if arg == 0 => String::new(),
                "Frame" => format!(">+<L(1)Repeat(+>+<L(1),{arg})R(1)[R(1)]+"),
                "L" => format!("Repeat(>,{})", CHUNK_SIZE * arg),
                "R" => format!("Repeat(<,{})", CHUNK_SIZE * arg),
                _ => unreachable!("Unexpected symbol `{mac:?}({arg})`"),
            };
            dst.push_str(&addition);
        } else {
            unreachable!("Unexpected macro {caps:#?}")
        }
    }
}

declare_regex_pass!(Simplification, r"\+-|-\+|><|<>");
impl Replacer for &Simplification {
    fn replace_append(&mut self, _caps: &regex::Captures<'_>, _dst: &mut String) {}
}

pub(crate) fn run_passes(mut src: String) -> String {
    let macro_pass = RepeatMacro::new();
    let chunk_pass = ChunkMacro::new();
    let simple_pass = Simplification::new();

    for _ in 0..PASS_LIMIT {
        let res = &src;
        let res = chunk_pass.run(&res);
        let res = macro_pass.run(&res);
        let res = simple_pass.run(&res);

        match res {
            Cow::Borrowed(_) => {
                return src;
            }
            Cow::Owned(next) => {
                src = next;
            }
        }
    }

    panic!("reached max pass iterations ({PASS_LIMIT}) final state:\n{src}");
}

#[cfg(test)]
mod test {
    use super::*;
    use expect_test::expect;

    #[test]
    fn test_replace() {
        let pass = RepeatMacro::new();
        expect![">>>>>"].assert_eq(&pass.run("Repeat(>, 5)"));
        expect![">>>>><<<<<"].assert_eq(&pass.run("Repeat(>, 5)Repeat(<, 5)"));
        expect!["xDxD"].assert_eq(&pass.run("Repeat(xD, 2)"));
        expect!["Repeat(, 2)"].assert_eq(&pass.run("Repeat(, 2)"));
        expect!["Repeat()"].assert_eq(&pass.run("Repeat()"));
        expect![""].assert_eq(&pass.run("Repeat(s, 0)"));
    }

    #[test]
    fn test_chunk_macro() {
        let pass = ChunkMacro::new();
        expect!["Repeat(>,4)"].assert_eq(&pass.run("L(1)"));
        expect!["Repeat(>,8)"].assert_eq(&pass.run("L(2)"));
        expect!["Repeat(<,4)"].assert_eq(&pass.run("R(1)"));
        expect!["Repeat(<,8)"].assert_eq(&pass.run("R(2)"));
        expect!["Use()"].assert_eq(&pass.run("Use()"));
        expect!["Repeat(>,1)-Repeat(<,1)"].assert_eq(&pass.run("Use(-)"));
        expect!["Repeat(>,3)++Repeat(<,3)"].assert_eq(&pass.run("Data(++)"));
        expect!["Repeat(>,2)++Repeat(<,2)"].assert_eq(&pass.run("Temp(++)"));
        expect![">+<L(1)Repeat(+>+<L(1),2)R(1)[R(1)]+"].assert_eq(&pass.run("Frame(2)"));
    }

    #[test]
    fn test_simplification() {
        let pass = Simplification::new();
        expect!["Unchainged"].assert_eq(&pass.run("Unchainged"));
        expect![">><"].assert_eq(&pass.run(">>><<"));
        expect!["+++"].assert_eq(&pass.run("++++-"));
        expect!["----"].assert_eq(&pass.run("-----+"));
        expect!["+-"].assert_eq(&pass.run("+><--+"));
    }

    #[test]
    fn rest_run_passes() {
        expect!["Unchainged"].assert_eq(&run_passes("Unchainged".to_string()));
        expect!["xy"].assert_eq(&run_passes("x>><<y".to_string()));
        expect!["ab"].assert_eq(&run_passes("a++--b".to_string()));
        expect![""].assert_eq(&run_passes("Repeat(>, 5)+-Repeat(<, 5)".to_string()));
    }
}
