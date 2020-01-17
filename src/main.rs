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
use crate::settings::CellType;
use crate::parser::Parser;

fn main() {
    let args = env::args();
    let settings = Settings::parse_args(args);

    match settings.cell_type {
        CellType::U8 => run_with_t::<u8>(settings),
        CellType::U16 => run_with_t::<u16>(settings),
        CellType::U32 => run_with_t::<u32>(settings),
    };
}

fn run_with_t<T>(settings: Settings)
    where T: Cell
{
    let code: Vec<Instruction<T>> = Parser::new(settings.parser_mode).parse_script(settings.get_src().as_str());

    let mut thing = Interpreter::new(&code, settings.buffer_size);
    thing.run();

    if settings.dump_mem {
        thing.dump_memory();
    }
}