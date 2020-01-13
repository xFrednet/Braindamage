use crate::{Instruction, Cell};
use std::str::Chars;

const OP_INC_INDEX: char = '>';
const OP_DEC_INDEX: char = '<';
const OP_INC_VALUE: char = '+';
const OP_DEC_VALUE: char = '-';
const OP_IO_READ: char = ',';
const OP_IO_WRITE: char = '.';
const OP_LOOP_START: char = '[';
const OP_LOOP_END: char = ']';
const OP_FILE_IO_READ: char = ';';
const OP_FILE_IO_WRITE: char = ':';
const END_OF_STRING: char = '\0';

pub fn parse_str<T>(script: &str) -> Vec<Instruction<T>>
    where
        T: Cell
{
    let script = script.to_ascii_lowercase();
    let mut instructions = script.chars();

    let ops = parse_block(&mut instructions, END_OF_STRING);
    aggregate_same(ops)
}

fn parse_block<T>(instructions : &mut Chars, block_end: char) -> Vec<Instruction<T>>
    where
        T: Cell
{
    let mut ops: Vec<Instruction<T>> = Vec::new();

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
            OP_INC_INDEX  => Instruction::IncreaseIndex(1),
            OP_DEC_INDEX  => Instruction::DecreaseIndex(1),
            OP_INC_VALUE  => Instruction::IncreaseValue(T::from(1u8)),
            OP_DEC_VALUE  => Instruction::DecreaseValue(T::from(1u8)),
            OP_IO_READ    => Instruction::IoRead,
            OP_IO_WRITE   => Instruction::IoWrite,
            OP_LOOP_START => Instruction::Loop(parse_block(instructions, OP_LOOP_END)),
            OP_FILE_IO_READ => Instruction::FileIoRead,
            OP_FILE_IO_WRITE => Instruction::FileIoWrite,
            _ => {
                Instruction::NoOp
            }
        })
    }

    ops
}

fn aggregate_same<T>(mut ops: Vec<Instruction<T>>) -> Vec<Instruction<T>>
    where
        T: Cell
{
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
            result.push(match last_item {
                Instruction::Loop(loop_ops) => Instruction::Loop(aggregate_same(loop_ops)),
                x => x
            });

            last_item = item;
        }
    }

    result.push(match last_item {
        Instruction::Loop(loop_ops) => Instruction::Loop(aggregate_same(loop_ops)),
        x => x
    });
    result
}