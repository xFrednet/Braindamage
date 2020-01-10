use crate::io::BraindamageIo;
use std::fs::File;
use std::io::{Write, Read};
use std::{slice, mem};

pub struct FileIo<'a, T> {
    file: &'a str,

    r_buffer: Vec<T>,
    r_index: usize,

    w_buffer: Vec<T>,
}

impl<'a, T> FileIo<'a, T>
    where T: Clone
{
    fn new(file_name: &'a str) -> Self {
        FileIo {
            file: file_name,

            r_buffer: Vec::new(),
            r_index: 0,

            w_buffer: Vec::new(),
        }
    }

    fn read_file(&mut self) {

        let mut f = File::open(self.file)?;
        let mut buffer = Vec::new();

        // read the whole file
        f.read_to_end(&mut buffer)?;

        let cell_data = &buffer[..];
        let read_cells = unsafe {
            let ptr = cell_data.as_ptr() as *const T;
            slice::from_raw_parts(ptr, cell_data.len())
        };

        self.r_buffer.clear();
        self.r_buffer.clone_from_slice(read_cells);
    }

    fn write_file(&mut self) -> T {
        let mut file = File::create(self.file).unwrap();

        let shared_data = &self.w_buffer[..];
        let bytes = unsafe {
            let len = shared_data.len() * mem::size_of::<T>();
            let ptr = shared_data.as_ptr() as *const u8;
            slice::from_raw_parts(ptr, len)
        };
        let res = file.write_all(bytes);

        self.w_buffer.clear();
        return T::from(if res.is_ok() { BraindamageIo::RET_IO_OK } else { BraindamageIo::RET_IO_ERR });
    }
}

impl<'a, T> BraindamageIo<T> for FileIo<'a, T>
    where T: Clone + PartialEq<T>
{
    fn read(&mut self) -> T {
        if self.r_index == 0 {
            self.read_file();
        }

        if self.r_index >= self.r_buffer.len() {
            self.r_index = 0;
            return (BraindamageIo::BUFFER_END_VALUE as T).clone();
        }

        let value = self.r_buffer[self.r_index].clone();
        self.r_index += 1;
        value
    }

    fn write(&mut self, value: T) -> T {
        if value == 0 as T {
            return self.write_file();
        }

        self.w_buffer.push(value.clone());
        value.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::io::file_io::FileIo;
    use crate::io::BraindamageIo;

    const FILE: String = String::from("Hello.txt");

    #[test]
    fn test_write() {
        let mut io = FileIo::new(FILE.as_str());

        io.write(70u8);
        io.write(0);

    }
}