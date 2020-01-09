use crate::{Operation};
use std::str::Chars;

const OP_INC_INDEX: char = '>';
const OP_DEC_INDEX: char = '<';
const OP_INC_VALUE: char = '+';
const OP_DEC_VALUE: char = '-';
const OP_IO_READ: char = ',';
const OP_IO_WRITE: char = '.';
const OP_LOOP_START: char = '[';
const OP_LOOP_END: char = ']';
const END_OF_STRING: char = '\0';

pub fn parse_str(script: &str) -> Vec<Operation> {
    let script = script.to_ascii_lowercase();
    let mut instructions = script.chars();

    parse_block(&mut instructions, END_OF_STRING)
}

fn parse_block(instructions : &mut Chars, block_end: char) -> Vec<Operation> {
    let mut ops = Vec::new();

    loop {
        // Get the char
        let inst = instructions.next();
        if inst.is_none() {
            if block_end != END_OF_STRING  {
                eprintln!("Error here, expected: {}", block_end);
            }

            break;
        }

        // Test for block end
        let inst = inst.unwrap();
        if inst == block_end {
            break;
        }

        ops.push(match inst {
            OP_INC_INDEX  => Operation::IncreaseIndex(1),
            OP_DEC_INDEX  => Operation::DecreaseIndex(1),
            OP_INC_VALUE  => Operation::IncreaseValue(1),
            OP_DEC_VALUE  => Operation::DecreaseValue(1),
            OP_IO_READ    => Operation::IoRead,
            OP_IO_WRITE   => Operation::IoWrite,
            OP_LOOP_START => Operation::Loop(parse_block(instructions, OP_LOOP_END)),
            _ => {
                eprintln!("Error the char \'{}\' is not a valid instruction.", inst);
                Operation::NoOp
            }
        })
    }

    ops
}