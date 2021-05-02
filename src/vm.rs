use crate::compiler::OpCode;
use libc::getchar;

pub struct VM {
    code: Vec<OpCode>,
    memory: Vec<u8>,
    memory_pointer: usize,
    instruction_pointer: u16,
    labels: Vec<u16>,
    output: String,
}

impl VM {
    pub fn new(code: Vec<OpCode>, to_alloc: usize) -> Self {
        Self {
            code,
            memory_pointer: 0,
            instruction_pointer: 0,
            output: String::new(),
            labels: vec![],
            memory: vec![0; to_alloc],
        }
    }
    fn run_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::Inc => {
                self.memory[self.memory_pointer] =
                    self.memory[self.memory_pointer].checked_add(1).unwrap_or(0)
            }
            OpCode::Dec => {
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer]
                    .checked_sub(1)
                    .unwrap_or(255)
            }
            OpCode::Next => {
                if self.memory_pointer + 1 >= self.memory.len() {
                    self.memory_pointer = 0;
                } else {
                    self.memory_pointer += 1;
                }
            }
            OpCode::Prev => {
                if self.memory_pointer as isize - 1 < 0 {
                    self.memory_pointer = self.memory.len() - 1;
                } else {
                    self.memory_pointer -= 1;
                }
            }
            OpCode::Display => self.output.push(self.memory[self.memory_pointer] as char),
            OpCode::Input => {
                let got = unsafe { getchar() };
                self.memory[self.memory_pointer] = got as u8;
            }
            OpCode::Label => self.labels.push(self.instruction_pointer),
            OpCode::Jmp => {
                if self.memory[self.memory_pointer] != 0 {
                    self.instruction_pointer = *self.labels.last().unwrap();
                } else {
                    self.labels.pop();
                }
            }
            OpCode::Nop => {}
        }
    }
    pub fn run(&mut self) -> (String, Vec<u8>, usize) {
        while (self.instruction_pointer as usize) < self.code.len() {
            let opcode = self.code[self.instruction_pointer as usize];
            self.run_opcode(opcode);
            self.instruction_pointer += 1;
        }

        (
            self.output.clone(),
            self.memory.clone(),
            self.memory_pointer,
        )
    }
}
