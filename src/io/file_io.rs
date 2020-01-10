use crate::io::BraindamageIo;
use std::fs::File;
use std::io::{Write, Read};
use std::{slice, mem};
use crate::io;

pub struct FileIo<'a, T> {
    file: &'a str,

    r_buffer: Vec<T>,
    r_index: usize,

    w_buffer: Vec<T>,
}

impl<'a, T> FileIo<'a, T>
    where T: Clone + PartialEq<T> + From<u8>
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
        self.r_buffer.clear();
        self.r_index = 0;

        let mut file = File::open(self.file);
        if file.is_ok() {
            let mut file = file.unwrap();
            let mut buffer = Vec::new();

            // read the whole file
            file.read_to_end(&mut buffer).unwrap();

            let cell_data = &buffer[..];
            let read_cells = unsafe {
                let ptr = cell_data.as_ptr() as *const T;
                slice::from_raw_parts(ptr, cell_data.len() / mem::size_of::<T>())
            };

            self.r_buffer.append(&mut read_cells.to_vec());
        } else {
            // TODO Add debug info.
        }
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
        let result : u8 = if res.is_ok() { io::RET_IO_OK } else { io::RET_IO_ERR };
        return result.into();
    }
}

impl<'a, T> BraindamageIo<T> for FileIo<'a, T>
    where T: Clone + PartialEq<T> + From<u8> + Default
{
    fn read(&mut self) -> T {
        if self.r_index == 0 {
            self.read_file();
        }

        if self.r_index >= self.r_buffer.len() {
            self.r_index = 0;
            return io::BUFFER_END_VALUE.clone().into();
        }

        let value = self.r_buffer[self.r_index].clone();
        self.r_index += 1;
        value
    }

    fn write(&mut self, value: T) -> T {
        if value == io::BUFFER_END_VALUE.into() {
            return self.write_file();
        }

        self.w_buffer.push(value.clone());
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::fmt::Debug;

    fn delete_file(file: &str) {
        remove_file(file);
    }

    #[test]
    fn test_read_empty() {
        let mut io : FileIo<u8> = FileIo::new("empty");

        assert_eq!(io.read(), 0u8);
        assert_eq!(io.read(), 0u8);

        let mut io : FileIo<u16> = FileIo::new("empty");

        assert_eq!(io.read(), 0u16);
        assert_eq!(io.read(), 0u16);

        let mut io : FileIo<u32> = FileIo::new("empty");

        assert_eq!(io.read(), 0u32);
        assert_eq!(io.read(), 0u32);
    }

    #[test]
    fn test_read_write() {
        const TEST_FILE : &'static str = "fileio_test_read_write.test";

        delete_file(TEST_FILE);
        test_read_write_type(TEST_FILE, &[1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8]);

        delete_file(TEST_FILE);
        test_read_write_type(TEST_FILE, &[1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16]);

        delete_file(TEST_FILE);
        test_read_write_type(TEST_FILE, &[1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32]);

        delete_file(TEST_FILE);
    }

    fn test_read_write_type<T>(file: &str, data: &[T])
        where T: Clone + PartialEq<T> + From<u8> + Default + Debug
    {
        let mut io: FileIo<T> = FileIo::new(file);

        // No file -> read 0
        assert_eq!(io.read(), 0.into());

        for var in data {
            // No file -> read 0
            assert_eq!(io.write(var.clone()), var.clone());
            assert_eq!(io.read(), 0u8.into());
        }

        assert_eq!(io.write(0u8.into()), 0u8.into());
        for var in data {
            assert_eq!(io.read(), var.clone());
        }
        assert_eq!(io.read(), 0u8.into());
        for var in data {
            assert_eq!(io.read(), var.clone());
        }
    }

    #[test]
    fn test_only_load_on_zero() {
        const TEST_FILE : &'static str = "fileio_only_load_on_zero.test";

        delete_file(TEST_FILE);

        let mut io : FileIo<u8> = FileIo::new(&TEST_FILE);

        // Write
        assert_eq!(io.read(), 0u8);
        assert_eq!(io.write(1u8), 1u8);
        assert_eq!(io.write(1u8), 1u8);
        assert_eq!(io.write(1u8), 1u8);
        assert_eq!(io.write(0u8), 0u8); // Save

        // Read once
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 0u8); // Read End of file
        assert_eq!(io.read(), 1u8);

        // Write a different value
        assert_eq!(io.write(9u8), 9u8);
        assert_eq!(io.write(9u8), 9u8);
        assert_eq!(io.write(9u8), 9u8);
        assert_eq!(io.write(0u8), 0u8); // Save

        // Read two more 1
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 0u8); // Read End of file

        // Read new values
        assert_eq!(io.read(), 9u8);
        assert_eq!(io.read(), 9u8);
        assert_eq!(io.read(), 9u8);
        assert_eq!(io.read(), 0u8); // Read End of file

        // Write a different value
        assert_eq!(io.write(5u8), 5u8);
        assert_eq!(io.write(5u8), 5u8);
        assert_eq!(io.write(5u8), 5u8);
        assert_eq!(io.write(0u8), 0u8); // Save

        // Read new values
        assert_eq!(io.read(), 5u8);
        assert_eq!(io.read(), 5u8);
        assert_eq!(io.read(), 5u8);
        assert_eq!(io.read(), 0u8); // Read End of file

        delete_file(TEST_FILE);
    }

    #[test]
    fn test_clear_file_on_small_buffer() {
        const TEST_FILE : &'static str = "fileio_test_clear_file_on_small_buffer.test";

        delete_file(TEST_FILE);

        let mut io : FileIo<u8> = FileIo::new(&TEST_FILE);

        // Write 111
        assert_eq!(io.read(), 0u8);
        assert_eq!(io.write(1u8), 1u8);
        assert_eq!(io.write(1u8), 1u8);
        assert_eq!(io.write(1u8), 1u8);
        assert_eq!(io.write(0u8), 0u8); // Save

        // Write 8
        assert_eq!(io.write(8u8), 8u8);
        assert_eq!(io.write(0u8), 0u8); // Save

        // Read 80
        assert_eq!(io.read(), 8u8);
        assert_eq!(io.read(), 0u8); // Read End of file
        assert_eq!(io.read(), 8u8);
        assert_eq!(io.read(), 0u8); // Read End of file
        assert_eq!(io.read(), 8u8);
        assert_eq!(io.read(), 0u8); // Read End of file

        delete_file(TEST_FILE);
    }

}