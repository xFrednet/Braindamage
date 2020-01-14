//    +,-
//   {^.°}
//   <[@]>
//    : ;
//
// Meet BDB the BrainDamageBot he is here to make sure that
// all code is awesome and readable

mod settings;
pub use settings::Settings;

mod parser;
mod operations;
pub use operations::Instruction;

mod buffer;
mod cell;
pub use cell::Cell;
mod interpreter;
use interpreter::Interpreter;

use std::env;

pub const ARRAY_SIZE: usize = 30_000;

fn main() {
    let args = env::args();
    let settings = Settings::parse_args(args);
    if settings.is_none() {
        return;
    }
    let settings = settings.unwrap();

    let code: Vec<Instruction<u8>> = parser::parse_str(settings.get_src().as_str());

    let mut thing = Interpreter::new(&code);
    thing.run();

    if settings.dump_mem {
        thing.dump_memory();
    }
}
