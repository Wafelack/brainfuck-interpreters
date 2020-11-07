mod parser;
mod scanner;

use parser::parser::parse;
use scanner::scanner::Scanner;

fn usage() {
    eprintln!("Usage : brainrust <file>");
    std::process::exit(-56);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage();
    }

    if !std::path::Path::new(&args[1]).exists() {
        eprintln!("File not found");
        usage();
    }

    let source = std::fs::read_to_string(&args[1]).unwrap();

    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens();

    let ccode = parse(tokens);
    let path = std::path::PathBuf::from(&args[1]);

    let fname = path.file_stem().unwrap();

    let fullname = format!("{}.c", fname.to_str().unwrap());
    println!("{}", fullname);
}
