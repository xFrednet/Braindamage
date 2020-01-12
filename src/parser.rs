use crate::{Operation, Cell};
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

pub fn parse_str<T>(script: &str) -> Vec<Operation<T>>
    where
        T: Cell
{
    let script = script.to_ascii_lowercase();
    let mut instructions = script.chars();

    let ops = parse_block(&mut instructions, END_OF_STRING);
    aggregate_same(ops)
}

fn parse_block<T>(instructions : &mut Chars, block_end: char) -> Vec<Operation<T>>
    where
        T: Cell
{
    let mut ops: Vec<Operation<T>> = Vec::new();

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
            OP_INC_VALUE  => Operation::IncreaseValue(T::from(1u8)),
            OP_DEC_VALUE  => Operation::DecreaseValue(T::from(1u8)),
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

fn aggregate_same<T>(mut ops: Vec<Operation<T>>) -> Vec<Operation<T>>
    where
        T: Cell
{
    let mut result = Vec::new();
    let mut iter = ops.drain(0..);
    let mut last_item = iter.next().unwrap_or(Operation::NoOp);

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
                Operation::Loop(loop_ops) => Operation::Loop(aggregate_same(loop_ops)),
                x => x
            });

            last_item = item;
        }
    }

    result.push(match last_item {
        Operation::Loop(loop_ops) => Operation::Loop(aggregate_same(loop_ops)),
        x => x
    });
    result
}