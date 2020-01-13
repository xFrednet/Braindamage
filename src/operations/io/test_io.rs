use crate::operations::io::BraindamageIo;
use crate::Cell;

pub struct TestIo<T: Cell> {
    read_data: Vec<T>,
    read_index: usize,
    loop_read_data: bool,

    expected_data: Vec<T>,
    expected_data_index: usize
}

impl<T> BraindamageIo<T> for TestIo<T>
    where T: Cell
{
    fn read(&mut self) -> T {
        if self.read_index >= self.read_data.len() {
            // Reset index if requested
            if self.loop_read_data {
                self.read_index = 0;
            } else {
                panic!("Data was requested after the read_buffer has ended.")
            }
        }

        self.read_index = self.read_index + 1;
        self.read_data[self.read_index - 1]
    }

    fn write(&mut self, value: T) -> T {
        if self.expected_data_index < self.expected_data.len() {
            self.expected_data_index += 1;
            let expect = self.expected_data[self.expected_data_index - 1];
            assert_eq!(value, expect);
        } else {
            panic!("Data was written after the expected_data was exceeded.");
        };

        value
    }
}

impl<T> TestIo<T>
    where T: Cell
{
    pub fn new(read_data: Vec<T>, loop_read_data: bool, expected_data: Vec<T>) -> Self {
        TestIo {
            read_data,
            read_index: 0,
            loop_read_data,
            expected_data,
            expected_data_index: 0
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_read_done() && self.is_write_done()
    }

    pub fn is_read_done(&self) -> bool {
        self.read_index == self.read_data.len() || self.loop_read_data
    }

    pub fn is_write_done(&self) -> bool{
        self.expected_data_index == self.expected_data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_no_loop() {
        let mut io: TestIo<u8> = TestIo::new(
            vec![0, 1, 2],
            false,
            Vec::new());
        assert!(io.is_write_done());
        assert!(!io.is_read_done());
        assert!(!io.is_done());
        assert_eq!(io.read(), 0u8);
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 2u8);
        assert!(io.is_read_done());
        assert!(io.is_done());
        assert!(std::panic::catch_unwind(move || io.read()).is_err());

        let mut io: TestIo<u8> = TestIo::new(
            vec![9, 6, 3],
            false,
            Vec::new());
        assert!(io.is_write_done());
        assert!(!io.is_read_done());
        assert!(!io.is_done());
        assert_eq!(io.read(), 9u8);
        assert_eq!(io.read(), 6u8);
        assert_eq!(io.read(), 3u8);
        assert!(io.is_read_done());
        assert!(io.is_done());
        assert!(std::panic::catch_unwind(move || io.read()).is_err());

        let mut io: TestIo<u8> = TestIo::new(
            Vec::new(),
            false,
            Vec::new());
        assert!(io.is_write_done());
        assert!(io.is_read_done());
        assert!(io.is_done());
        assert!(std::panic::catch_unwind(move || io.read()).is_err());
    }

    #[test]
    fn test_read_loop() {
        let mut io: TestIo<u8> = TestIo::new(
            vec![0, 1, 2],
            true,
            Vec::new());
        assert!(io.is_write_done());
        assert!(io.is_read_done());
        assert!(io.is_done());
        assert_eq!(io.read(), 0u8);
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 2u8);

        assert_eq!(io.read(), 0u8);
        assert_eq!(io.read(), 1u8);
        assert_eq!(io.read(), 2u8);

        let mut io: TestIo<u8> = TestIo::new(
            vec![9, 6, 3],
            true,
            Vec::new());
        assert!(io.is_write_done());
        assert!(io.is_read_done());
        assert!(io.is_done());
        assert_eq!(io.read(), 9u8);
        assert_eq!(io.read(), 6u8);
        assert_eq!(io.read(), 3u8);

        assert_eq!(io.read(), 9u8);
        assert_eq!(io.read(), 6u8);
        assert_eq!(io.read(), 3u8);
    }

    #[test]
    fn test_write() {
        let mut io: TestIo<u8> = TestIo::new(
            Vec::new(),
            true,
            vec![1, 2, 3, 4]);
        assert!(!io.is_write_done());
        assert!(io.is_read_done());
        assert!(!io.is_done());
        assert_eq!(io.write(1u8), 1u8);
        assert_eq!(io.write(2u8), 2u8);
        assert_eq!(io.write(3u8), 3u8);
        assert_eq!(io.write(4u8), 4u8);
        assert!(io.is_write_done());
        assert!(io.is_done());
        assert!(std::panic::catch_unwind(move || io.write(4u8)).is_err());

        let mut io: TestIo<u8> = TestIo::new(
            Vec::new(),
            true,
            vec![8, 0]);
        assert!(!io.is_write_done());
        assert!(io.is_read_done());
        assert!(!io.is_done());
        assert_eq!(io.write(8u8), 8u8);
        assert_eq!(io.write(0u8), 0u8);
        assert!(io.is_write_done());
        assert!(io.is_done());
        assert!(std::panic::catch_unwind(move || io.write(0u8)).is_err());
    }

}