//    +,-
//   {^.°}
//   <[@]>
//    : ;
//
// Meet BDB the BrainDamageBot he is here to make sure that
// all code is awesome and readable

mod cell;
mod buffer;
pub use cell::Cell;

mod operations;
pub use operations::Instruction;

mod parser;
mod ops_walker;

pub const ARRAY_SIZE: usize = 30_000;

fn main() {
    let code = parser::parse_str("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.[-]--");
    //let code = parser::parse_str("--");
    //println!("OpCode: {:?}", code);

    ops_walker::exec_ops(&code);
}
