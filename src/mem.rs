use std::fmt::Write;
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

const HEAP_CELL_SIZE: usize = 3;
const STACK_CELL_SIZE: usize = 2;

#[derive(Debug, Copy, Clone)]
pub struct MemInfo {
    layout: Layout,
}

#[derive(Debug, Copy, Clone)]
enum Layout {
    Uniform(usize),
    StackMachine { heap: usize, stack: usize },
}

impl MemInfo {
    pub fn size(&self) -> usize {
        match self.layout {
            Layout::Uniform(size) => size,
            Layout::StackMachine { heap, stack } => {
                (heap * HEAP_CELL_SIZE) + HEAP_CELL_SIZE + (stack * STACK_CELL_SIZE)
            },
        }
    }

    pub fn start_pos(&self) -> usize {
        match self.layout {
            Layout::StackMachine { heap, .. } => heap * HEAP_CELL_SIZE,
            _ => 0,
        }
    }

    pub fn print_mem(&self, mem: &[u8], head_idx: usize) -> String {
        fn to_hex_str(mem_indices: impl Iterator<Item = usize>, mem: &[u8], head_idx: usize) -> String {
            let mut bytes = String::new();
            for idx in mem_indices {
                let prefix = if idx == head_idx { '>' } else { ' ' };
                let value = mem[idx];
                write!(bytes, "{prefix}{value:02X}").unwrap();
            }
            bytes
        }

        fn to_txt_str(mem_indices: impl Iterator<Item = usize>, mem: &[u8]) -> String {
            let mut txt = String::new();
            for idx in mem_indices {
                let char_value = char::from(mem[idx]);
                if char_value.is_alphanumeric() {
                    txt.push(char_value);
                } else {
                    txt.push('.');
                }
            }
            txt
        }

        const ROW_SIZE: usize = 16;
        const CHUNK_SIZE: usize = 8;
        const MEM_SIZE: usize = (ROW_SIZE / CHUNK_SIZE) * (CHUNK_SIZE * 3 + 4);

        let mut buffer = String::new();
        match self.layout {
            Layout::Uniform(_) => {
                let range = 0..mem.len();
                for (row_num, row) in range.chunks(ROW_SIZE).into_iter().enumerate() {
                    let start_addr = row_num * ROW_SIZE;

                    let mut bytes = String::with_capacity(MEM_SIZE);
                    let mut text = String::with_capacity(MEM_SIZE);
                    for mut chunk in &row.chunks(CHUNK_SIZE) {
                        let start = chunk.next().unwrap();
                        let end = chunk.last().unwrap_or(start);

                        write!(bytes, "  {}", to_hex_str(start..=end, mem, head_idx)).unwrap();
                        write!(text, "  {}", to_txt_str(start..=end, mem)).unwrap();
                    }

                    writeln!(buffer, "{start_addr:08X} |{bytes}  |{text}").unwrap();
                }
            },
            Layout::StackMachine { .. } => todo!(),
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

        // Stack Machine Memory
        let re = Regex::new(r#"(\d*)H(\d*)S"#).unwrap();
        if let Some(caps) = re.captures(s) {
            return Ok(Self {
                layout: Layout::StackMachine {
                    heap: caps[0].parse().unwrap(),
                    stack: caps[1].parse().unwrap(),
                },
            });
        }

        Err(format!("Invalid memory config: `{s:?}`"))
    }
}
