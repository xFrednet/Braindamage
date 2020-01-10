mod buffer;

mod parser;

mod ops_walker;
mod operation;
pub use operation::Operation;

pub const ARRAY_SIZE: usize = 30_000;
pub type BufType = u8;
pub type PtrType = u32;

fn main() {
    let code = parser::parse_str("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.[-]--");
    //let code = parser::parse_str("--");
    //println!("OpCode: {:?}", code);

    ops_walker::exec_ops(&code);
}
