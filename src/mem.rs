use std::fmt::Write;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub struct MemInfo {
    layout: Layout,
}

#[derive(Debug, Copy, Clone)]
enum Layout {
    Uniform(usize),
}

impl MemInfo {
    pub fn size(&self) -> usize {
        match self.layout {
            Layout::Uniform(size) => size,
        }
    }

    pub fn start_pos(&self) -> usize {
        match self.layout {
            _ => 0,
        }
    }

    pub fn print_mem(&self, mem: &[u8]) -> String {
        const ROW_SIZE: usize = 16;
        const CHUNK_SIZE: usize = 8;
        const MEM_SIZE: usize = (ROW_SIZE / CHUNK_SIZE) * (CHUNK_SIZE * 3 + 4);

        let mut buffer = String::new();
        match self.layout {
            Layout::Uniform(_) => {
                for (row_num, row) in mem.chunks(ROW_SIZE).enumerate() {
                    let start_addr = row_num * ROW_SIZE;

                    let mut bytes = String::with_capacity(MEM_SIZE);
                    let mut text = String::with_capacity(MEM_SIZE);
                    for chunk in row.chunks(CHUNK_SIZE) {
                        write!(bytes, " ").unwrap();
                        write!(text, " ").unwrap();

                        for value in chunk {
                            write!(bytes, " {value:02X}").unwrap();

                            let char_value = char::from(*value);
                            if char_value.is_alphanumeric() {
                                text.push(char_value);
                            } else {
                                text.push('.');
                            }
                        }
                    }

                    writeln!(buffer, "{start_addr:08X} |{bytes} |{text}").unwrap();
                }
            },
        }

        buffer
    }
}

impl FromStr for MemInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Uniform Memory
        let re = Regex::new(r#"(\d*)"#).unwrap();
        if let Some(caps) = re.captures(s) {
            return Ok(Self {
                layout: Layout::Uniform(caps[0].parse().unwrap()),
            });
        }

        Err(format!("Invalid memory config: `{s:?}`"))
    }
}
