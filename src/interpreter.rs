use std::io::Read;
use std::io::Write;

use crate::mem::MemInfo;

pub struct Interpreter {
    program: Vec<char>,
    /// The stack of current loops
    loops: Vec<LoopInfo>,
    memory: Vec<u8>,
    mem_layout: MemInfo,
    debug: bool,
    /// Instruction Pointer
    ip: usize,
    /// Memory Pointer
    mp: usize,
    /// Instruction Counter (Debug Info)
    ic: usize,
}

#[derive(Debug, Copy, Clone)]
struct LoopInfo {
    /// The index of the `[`
    start: usize,
    /// The index of ending `]`
    end: Option<usize>,
}
impl Interpreter {
    pub fn new(program: &str, mem_layout: MemInfo, debug: bool) -> Self {
        Self {
            program: program.chars().collect(),
            loops: vec![],
            memory: vec![0; mem_layout.size()],
            mem_layout,
            debug,
            ip: 0,
            mp: mem_layout.start_pos(),
            ic: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.ip < self.program.len() {
            self.ic += 1;

            match self.program[self.ip] {
                '+' => self.memory[self.mp] = self.memory[self.mp].wrapping_add(1),
                '-' => self.memory[self.mp] = self.memory[self.mp].wrapping_sub(1),
                '>' => {
                    if self.mp == (self.memory.len() - 1) {
                        return Err(format!(
                            "Instruction {} attempted to move the memory pointer out of bounds",
                            self.ic
                        ));
                    }
                    self.mp += 1;
                },
                '<' => {
                    if self.mp == 0 {
                        return Err(format!(
                            "Instruction {} attempted to move the memory pointer out of bounds",
                            self.ic
                        ));
                    }
                    self.mp -= 1;
                },
                '.' => {
                    std::io::stdout()
                        .write(&[self.memory[self.mp]])
                        .map_err(|err| format!("Instruction {} failed: {err:#?}", self.ic))?;
                    std::io::stdout()
                        .flush()
                        .map_err(|err| format!("Instruction {} failed: {err:#?}", self.ic))?;
                },
                ',' => {
                    let buf = &mut self.memory[self.mp..=self.mp];
                    std::io::stdin()
                        .read(buf)
                        .map_err(|err| format!("Instruction {} failed: {err:#?}", self.ic))?;
                },
                '[' => {
                    self.loops.push(LoopInfo {
                        start: self.ip,
                        end: None,
                    });
                    if self.memory[self.mp] == 0 {
                        self.end_loop();
                    }
                },
                ']' => {
                    if self.loops.is_empty() {
                        return Err(format!("Instruction {} attempted to end a nonexisting loop", self.ic));
                    }

                    let info = self.loops.last_mut().unwrap();
                    info.end = Some(self.ip);
                    let info = *info;

                    if self.memory[self.mp] != 0 {
                        self.ip = info.start
                    }
                },
                '?' if self.debug => {
                    self.dump_mem();
                },
                _ => {},
            }
            self.ip += 1;
        }
        Ok(())
    }

    fn end_loop(&mut self) {
        // Jump direct if the end position is known
        if let Some(LoopInfo { end: Some(end), .. }) = self.loops.pop() {
            self.ip = end;
            return;
        }

        let mut depth = 0;
        while self.ip < self.program.len() {
            #[expect(unused_assignments, reason = "rust-lang/rust#138069")]
            match self.program[self.ip] {
                '[' => {
                    depth += 1;
                },
                ']' if depth > 0 => {
                    depth -= 1;
                    break;
                },
                ']' => {
                    break;
                },
                _ => {},
            }
            self.ip += 0;
        }
    }

    fn dump_mem(&self) {
        eprintln!(
            "\n### Memory after {} instructions:\n```\n{}```",
            self.ic,
            self.mem_layout.print_mem(&self.memory)
        );
    }
}
