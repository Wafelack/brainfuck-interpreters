use crate::{Error, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpCode {
    Display,
    Inc,
    Dec,
    Next,
    Prev,
    Jmp,
    Label,
    Input,
    Nop,
}

use std::fmt::{self, Display, Formatter};

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Display => "DISPLAY",
                Self::Input => "INPUT",
                Self::Inc => "INC",
                Self::Dec => "DEC",
                Self::Next => "NEXT",
                Self::Prev => "PREV",
                Self::Jmp => "JMP",
                Self::Label => "LABEL",
                Self::Nop => "NOP",
            }
        )
    }
}

#[derive(Clone)]
pub struct Compiler {
    input: String,
    output: Vec<OpCode>,
    current: u16,
    pos: usize,
    line: usize,
    to_alloc: usize,
}

impl Compiler {
    pub fn new(input: impl ToString) -> Self {
        Self {
            input: input.to_string(),
            output: vec![],
            current: 0,
            pos: 0,
            line: 1,
            to_alloc: 1,
        }
    }
    fn compile_char(&mut self, c: char) -> Result<OpCode> {
        match c {
            '\n' => {
                self.line += 1;
                self.pos = 0;
                Ok(OpCode::Nop)
            }
            '+' => Ok(OpCode::Inc),
            '-' => Ok(OpCode::Dec),
            '>' => {
                self.to_alloc += 1;
                Ok(OpCode::Next)
            }
            '<' => Ok(OpCode::Prev),
            '.' => Ok(OpCode::Display),
            ',' => Ok(OpCode::Input),
            '[' => {
                self.current += 1;
                Ok(OpCode::Label)
            }
            ']' => {
                if self.current < 1 {
                    Err(Error(format!(
                        "{}:{} | Mismatched closing delimiter.",
                        self.line, self.pos
                    )))
                } else {
                    self.current -= 1;
                    Ok(OpCode::Jmp)
                }
            }
            _ => Ok(if c == '\n' {
                self.line += 1;
                self.pos = 0;
                OpCode::Nop
            } else {
                OpCode::Nop
            }),
        }
    }
    pub fn compile(&mut self) -> Result<(Vec<OpCode>, usize)> {
        for chr in self.input.clone().chars() {
            self.pos += 1;
            let to_push = self.compile_char(chr)?;
            self.output.push(to_push);
        }

        if self.current != 0 {
            Err(Error(format!(
                "{}:{} | {} loops were not closed.",
                self.line, self.pos, self.current
            )))
        } else {
            Ok((self.output.clone(), self.to_alloc))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loops() -> Result<()> {
        let (opcodes, _) = Compiler::new("[[[]]]").compile()?;
        assert_eq!(
            opcodes,
            vec![
                OpCode::Label,
                OpCode::Label,
                OpCode::Label,
                OpCode::Jmp,
                OpCode::Jmp,
                OpCode::Jmp
            ]
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn failing_loops() {
        let (_, _) = Compiler::new("[[[]]").compile().unwrap();
    }

    #[test]
    fn values() -> Result<()> {
        let (opcodes, _) = Compiler::new("+--+").compile()?;
        assert_eq!(
            opcodes,
            vec![OpCode::Inc, OpCode::Dec, OpCode::Dec, OpCode::Inc]
        );
        Ok(())
    }

    #[test]
    fn pointers() -> Result<()> {
        let (opcodes, _) = Compiler::new(">><<").compile()?;
        assert_eq!(
            opcodes,
            vec![OpCode::Next, OpCode::Next, OpCode::Prev, OpCode::Prev]
        );
        Ok(())
    }

    #[test]
    fn real_world() -> Result<()> {
        let (opcodes, _) = Compiler::new(".,").compile()?;
        assert_eq!(opcodes, vec![OpCode::Display, OpCode::Input]);
        Ok(())
    }

    #[test]
    fn comments() -> Result<()> {
        let (opcodes, _) = Compiler::new("Hello World !").compile()?;
        assert_eq!(opcodes, vec![OpCode::Nop; opcodes.len()]);
        Ok(())
    }
}
