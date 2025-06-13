use std::borrow::Cow;

use regex::{Regex, Replacer};

const PASS_LIMIT: u32 = 100;

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

declare_regex_pass!(ConstIntPass, r"u8\.const\s*(?<lit>\d+)");
impl Replacer for &ConstIntPass {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        // The pointer should be at the start of the stack
        let lit_str = &caps["lit"];
        let lit = lit_str.parse().unwrap();

        // Comment
        dst.push_str("// u8-const ");
        dst.push_str(lit_str);
        dst.push('\n');

        // Go to new cell
        dst.push_str(&">".repeat(crate::mem::STACK_CELL_SIZE));

        // Mark the stack cell as filled
        dst.push('+');

        // Fill the data
        dst.push('>'); // To data

        dst.push_str(&"+".repeat(lit));
        dst.push('<'); // To marker

        // End line
        dst.push('\n');
    }
}

declare_regex_pass!(IntAddPass, r"u8\.add");
impl Replacer for &IntAddPass {
    fn replace_append(&mut self, _caps: &regex::Captures<'_>, dst: &mut String) {
        // Comment
        dst.push_str("// u8-add\n");
        dst.push_str(">[-<<+>>]<-<<\n");
    }
}

pub(crate) fn run_wat_passes(src: String) -> String {
    let src = ConstIntPass::new().run(&src);
    let src = IntAddPass::new().run(&src);
    src.to_string()
}

declare_regex_pass!(Simplification, r"\+-|-\+|><|<>");
impl Replacer for &Simplification {
    fn replace_append(&mut self, _caps: &regex::Captures<'_>, _dst: &mut String) {}
}

pub(crate) fn run_base_passes(mut src: String) -> String {
    let simple_pass = Simplification::new();

    for _ in 0..PASS_LIMIT {
        let res = &src;
        let res = simple_pass.run(&res);

        match res {
            Cow::Borrowed(_) => {
                return src;
            },
            Cow::Owned(next) => {
                src = next;
            },
        }
    }

    panic!("reached max pass iterations ({PASS_LIMIT}) final state:\n{src}");
}

#[cfg(test)]
mod test {
    use super::*;
    use expect_test::expect;

    #[test]
    fn test_simplification() {
        let pass = Simplification::new();
        expect!["Unchanged"].assert_eq(&pass.run("Unchanged"));
        expect![">><"].assert_eq(&pass.run(">>><<"));
        expect!["+++"].assert_eq(&pass.run("++++-"));
        expect!["----"].assert_eq(&pass.run("-----+"));
        expect!["+-"].assert_eq(&pass.run("+><--+"));
    }
}
