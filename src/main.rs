mod compiler;
mod vm;

use clap::{App, Arg};
use compiler::{Compiler, OpCode};
use std::{fs, process::exit, time::Instant};
use vm::VM;

fn compile_error(err: String) -> ! {
    eprint!("\x1b[0;31mCompile ");
    error(err)
}

fn error(err: String) -> ! {
    eprintln!("\x1b[0;31mError\x1b[0m: {}", err);
    exit(1)
}

fn main() {
    let matches = App::new("brainrust")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("A brainfuck virtual machine.")
        .arg(
            Arg::with_name("file")
                .index(1)
                .takes_value(true)
                .required(true)
                .help("The source file to compile."),
        )
        .arg(
            Arg::with_name("clean")
                .short("c")
                .long("clean")
                .help("Remove NOP instructions from bytecode."),
        )
        .arg(
            Arg::with_name("dbg_level")
                .short("g")
                .long("dbg-level")
                .takes_value(true)
                .help("The debug level to use. Defaults to 0."),
        )
        .get_matches();

    let dbg_level = match matches.value_of("dbg_level") {
        Some(val) => match val.parse::<u32>() {
            Ok(u) => u,
            _ => 0,
        },
        _ => 0,
    };
    let filename = matches.value_of("file").unwrap();

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => error(format!("Failed to read `{}`: {}.", filename, e)),
    };
    println!("* \x1b[0;32mCompiling\x1b[0m `{}`...", filename);
    let start = Instant::now();
    let (bytecode, to_alloc) = match Compiler::new(content).compile() {
        Ok(b) => b,
        Err(e) => compile_error(e.0),
    };
    let elapsed = start.elapsed();
    println!(
        "  \x1b[0;32mCompiled\x1b[0m in {} seconds.",
        elapsed.as_secs_f32()
    );

    let bytecode = if matches.is_present("clean") {
        let len = bytecode.len();
        if dbg_level > 1 {
            println!("* \x1b[0;32mCleaning\x1b[0m NOP instructions ...");
        }
        let bytecode = bytecode
            .into_iter()
            .filter(|op| *op != OpCode::Nop)
            .collect::<Vec<_>>();
        if dbg_level > 1 {
            println!(
                "  \x1b[0;32mCleaned\x1b[0m {} NOP instructions.",
                len - bytecode.len()
            );
        }
        bytecode
    } else {
        bytecode
    };

    if dbg_level > 0 {
        println!("[INSTRUCTIONS]");
        bytecode.iter().enumerate().for_each(|(idx, opcode)| {
            println!("{:04x}: {}", idx, opcode);
        });
    }
    println!(
        "* \x1b[0;32mRunning\x1b[0m {} instructions...",
        bytecode.len()
    );
    let start = Instant::now();
    let (output, memory, pointer) = VM::new(bytecode, to_alloc).run();
    let elapsed = start.elapsed();
    println!("{}", output);
    println!(
        "  \x1b[0;32mDone\x1b[0m in {} seconds.",
        elapsed.as_secs_f32()
    );

    if dbg_level > 1 {
        println!("[MEMORY]");
        memory.into_iter().enumerate().for_each(|(idx, val)| {
            println!("0x{:04x}: 0x{:02x}", idx, val);
        });
    }
    if dbg_level > 1 {
        println!("MEMORY_POINTER: {:04x}.", pointer);
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(String);
