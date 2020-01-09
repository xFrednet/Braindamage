mod buffer;

mod parser;

mod ops_walker;

pub const ARRAY_SIZE: usize = 30_000;
pub type BufType = u8;
pub type PtrType = u32;

#[repr(u8)]
#[derive(Debug)]
pub enum Operation {
    NoOp,
    IncreaseIndex(usize),
    DecreaseIndex(usize),
    IncreaseValue(BufType),
    DecreaseValue(BufType),
    IoRead,
    IoWrite,
    Loop(Vec<Operation>),
}

fn main() {
    let code = parser::parse_str("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.");
    //println!("OpCode: {:?}", code);

    ops_walker::exec_ops(&code);
}
