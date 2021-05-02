mod compiler;
mod vm;

use compiler::Compiler;
use vm::VM;
use std::io::{self, Write};

fn main() -> Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let (opcodes, to_alloc)= Compiler::new(buffer.trim()).compile()?;
        VM::new(opcodes, to_alloc).run();
        io::stdout().flush().unwrap();
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(String);
