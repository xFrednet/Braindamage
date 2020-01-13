use crate::operations::io::BraindamageIo;
use crate::Cell;
use std::io;

pub struct ConsoleIo;

impl<T> BraindamageIo<T> for ConsoleIo
    where T: Cell
{
    fn read(&mut self) -> T {
        let mut buffer = String::new();
        let _result = io::stdin().read_line(&mut buffer);
        buffer.bytes().next().unwrap_or_default().into()
    }

    fn write(&mut self, value: T) -> T {
        let c = value.to_char();
        print!("{}", c);
        value
    }
}

impl ConsoleIo {
    pub fn new() -> Self {
        ConsoleIo {

        }
    }
}