use crate::{Instruction, Cell};

mod lexer;
use lexer::Lexer;

pub fn parse_str<T>(script: &str) -> Vec<Instruction<T>>
    where
        T: Cell
{
    let mut lexer = Lexer::default();
    lexer.lex_string(script);
    lexer.finish()

    //aggregate_same(ops)
}

#[allow(dead_code)]
fn aggregate_same<T>(mut ops: Vec<Instruction<T>>) -> Vec<Instruction<T>>
    where
        T: Cell
{
    // This causes currently an error because it changed the indices of loop start and ends by removing instructions
    // but this is not implemented here
    let mut result = Vec::new();
    let mut iter = ops.drain(0..);
    let mut last_item = iter.next().unwrap_or(Instruction::NoOp);

    loop {
        let item = iter.next();
        if item.is_none() {
            break;
        }
        let item = item.unwrap();

        // Test for join
        if last_item.can_join(&item) {
            last_item = last_item.join(item);
        } else {
            result.push(last_item);
            last_item = item;
        }
    }

    result.push(last_item);
    result
}