use crate::{Operation, BufType};
use crate::buffer::BrainfuckBuffer;


pub fn exec_ops(ops: &Vec<Operation>) {
    let mut buffer = BrainfuckBuffer::default();
    walk_ops(ops, &mut buffer);
    println!("{:?}", buffer);
}

fn walk_ops(ops: &Vec<Operation>, buffer: &mut BrainfuckBuffer) {
    for op in ops {
        match op {
            Operation::NoOp => {},
            Operation::IncreaseIndex(value) => {buffer.inc_index(*value)},
            Operation::DecreaseIndex(value) => {buffer.dec_index(*value)},
            Operation::IncreaseValue(value) => {buffer.inc_value(*value)},
            Operation::DecreaseValue(value) => {buffer.dec_value(*value)},
            Operation::IoRead => {buffer.read()},
            Operation::IoWrite => {buffer.write()},
            Operation::Loop(loop_ops) => {
                let break_value: BufType = BufType::default();
                while buffer.get_value(buffer.index) != break_value {
                    walk_ops(loop_ops, buffer)
                }
            }
        }
    }
}
