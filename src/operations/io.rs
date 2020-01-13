use crate::Cell;

pub mod file_io;
pub mod console_io;
pub mod test_io;

const BUFFER_END_VALUE : u8 = 0;
const RET_IO_OK: u8 = 0;
const RET_IO_ERR: u8 = 1;

pub trait BraindamageIo<T: Cell> {
    fn read(&mut self) -> T;

    fn write(&mut self, value: T) -> T;
}