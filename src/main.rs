mod compiler;

use compiler::Compiler;
use std::io;

fn main() -> Result<()> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let opcodes = Compiler::new(buffer.trim()).compile()?;
        println!("\n[INSTRUCTIONS]");
        opcodes.0.into_iter().enumerate().for_each(|(idx, opcode)| {
            println!("0x{:04x}: {}", idx, opcode)
        });
        println!("\nTo allocate: 0x{:02x}.", opcodes.1);
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(String);
