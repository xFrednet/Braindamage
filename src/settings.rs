use std::fs::File;
use std::io::Read;
use std::process;
use crate::parser::{ParserMode};

enum ArgState {
    SkipExeFile,
    Start,
    FileArg,
    SrcArg,
    CellArg,
    BufferArg
}

#[derive(Debug, PartialEq)]
pub enum CellType {
    U8,
    U16,
    U32,
}

#[derive(Debug)]
pub struct Settings {
    pub src: String,
    pub parser_mode: ParserMode,
    pub dump_mem: bool,
    pub cell_type: CellType,
    pub buffer_size: usize
}

impl Settings {
    pub fn parse_args<T>(args: T) -> Settings
        where T: Iterator<Item=String>
    {
        let mut settings = Self::init_default();

        let mut mode = ArgState::SkipExeFile;
        for arg in args {
            match mode {
                ArgState::SkipExeFile => mode = ArgState::Start,
                ArgState::Start => {
                    match arg.as_str() {
                        "-f" | "--file" => mode = ArgState::FileArg,
                        "-s" | "--src" => mode = ArgState::SrcArg,
                        "-h" | "--help" => {
                            print_help();
                            process::exit(0);
                        }

                        "-d" | "--debug" => settings.parser_mode = ParserMode::Debug,
                        "--dump" => settings.dump_mem = true,

                        "-c" | "--cell" => mode = ArgState::CellArg,
                        "-b" | "--buffer" => mode = ArgState::BufferArg,
                        x => {
                            println!("Unknown argument: {}", x);
                            print_help();
                            process::exit(0);
                        }
                    }
                },
                ArgState::FileArg => {
                    let src = load_source_file(arg);
                    if src.is_none() {
                        process::exit(-1);
                    }

                    settings.src = src.unwrap();
                    mode = ArgState::Start;
                },
                ArgState::SrcArg => {
                    settings.src = arg.clone();
                    mode = ArgState::Start;
                },
                ArgState::CellArg => {
                    match arg.as_str() {
                        "u8" => settings.cell_type = CellType::U8,
                        "u16" => settings.cell_type = CellType::U16,
                        "u32" => settings.cell_type = CellType::U32,
                        _ => println!("Please enter a valid cell type. Valid types are: u8, u16, u32.")
                    }

                    mode = ArgState::Start;
                },
                ArgState::BufferArg => {
                    let size = arg.to_string().parse::<usize>();
                    if size.is_err() {
                        println!("Please enter a valid buffer size.");
                    } else {
                        settings.buffer_size = size.unwrap();
                    }

                    mode = ArgState::Start;
                },
            }
        }

        return settings;
    }

    fn init_default() -> Settings {
        Settings {
            src: String::default(),
            parser_mode: ParserMode::Release,
            dump_mem: false,
            cell_type: CellType::U8,
            buffer_size: 30_000
        }
    }

    pub fn get_src(&self) -> String {
        self.src.clone()
    }
}

fn load_source_file(file_name: String) -> Option<String> {
    let file = File::open(file_name.clone());
    if file.is_err() {
        println!("Unable to load the file: \"{}\"", file_name);
        return None;
    }

    let mut file = file.unwrap();
    let mut src = String::default();

    if file.read_to_string(&mut src).is_err() {
        println!("Unable to read the file: {}", file_name);
        return None;
    }

    return Some(src);
}

fn print_help() {
    println!("
braindamage [[--help | -h] | [--file | -f <src-file>] | [--src | -s <src-code>]]
            [-d | --debug] [--dump]

Main operation (Required):
    -s --src  <src-code>           Sets the provided string as the src.
    -f --file <src-file>           Sets the file content as the src.
    -h --help                      Prints this help message.

Debugging:
    -d --debug                  This enabled debugging instructions and disables parser optimisation.
       --dump                   Dumps out the memory buffer after execution.

Settings:
    -c --cell (u8 | u16 | u32)     This sets the type for the buffer cell. (Default: u8)
    -b --buffer <usize>            This defines the size of the buffer. (Default: 30000)
");
}
