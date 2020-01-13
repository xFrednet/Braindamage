//    +,-
//   {^.°}
//   <[@]>
//    : ;
//
// Meet BDB the BrainDamageBot he is here to make sure that
// all code is awesome and readable

mod interpreter;
mod buffer;
mod cell;
pub use cell::Cell;

mod operations;
pub use operations::Instruction;
use crate::interpreter::Interpreter;

mod parser;

pub const ARRAY_SIZE: usize = 30_000;

fn main() {
    let  code: Vec<Instruction<u8>> = parser::parse_str(
        "++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++:>+:+++++++::+++:<<++:>+++++++++++++++:>:+++:------:--------:<<+:<:[-]--<:;[.;]");
    //let code = parser::parse_str("--");
    //println!("OpCode: {:?}", code);

    let mut thing = Interpreter::new(&code);
    thing.run();

    //ops_walker::exec_ops(&code);
}
