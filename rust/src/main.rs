use libc::getchar;
use std::io::{stdin, Write, stdout};

pub struct BfVM<'a> {
   ip: usize,
   labels: Vec<usize>,
   memory: Vec<u8>,
   ptr: usize,
   code: &'a str,
}
impl<'a> BfVM<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            ip: 0,
            labels: vec![],
            memory: vec![0],
            ptr: 0,
        }
    }
    fn run_turn(&mut self) {
        self.ip += 1;
        if let Some(current) = self.code.chars().nth(self.ip - 1) {
            let pointed_val = self.memory[self.ptr];
            match current {
                '>' => {
                    if self.ptr + 1 >= self.memory.len() {
                        self.memory.push(0);
                    }
                    self.ptr += 1;
                }
                '<' => self.ptr = if self.ptr == 0 { self.memory.len() - 1 } else { self.ptr - 1 },
                '+' => self.memory[self.ptr] = pointed_val.checked_add(1).unwrap_or(0),
                '-' => self.memory[self.ptr] = pointed_val.checked_sub(1).unwrap_or(255),
                '[' => self.labels.push(self.ip),
                ']' => if pointed_val == 0 { self.labels.pop(); } else { self.ip = self.labels.last().and_then(|l| Some(*l)).unwrap_or(self.ip) },
                '.' => print!("{}", pointed_val as char),
                ',' => self.memory[self.ptr] = unsafe { getchar() } as u8,
                _ => {}
            }
        }
   }
    pub fn execute(mut self) {
        while self.ip < self.code.len() {
            self.run_turn();
        }
    }
}

fn main() {
    BfVM::new("++[->++++<]>+[->++++++++++<]>---.
			<++[->+++++++<]>.
			<++[->+++<]>+.
			<++[->----<]>-.
			<++++[->+++<]>.
			--.
			<++[->----<]>.
			<+++++++[->----------<]>+.
			<++++++++++[->++++++++<]>++++.
			-----.
			<++++++++++[->--------<]>+.
			<++++++++++[->+++++<]>+.
			<++++[->++++<]>.
			--.
			<+++++[->++<]>+.
			<+++++[->--<]>-.
			<++++++++[->----<]>+.
			++++.
			>+++++[->++<]>.").execute();
    loop {
        BfVM::new("+++[->++++<]>[->+++++<]>++.<++++++[->-----<]>.").execute();
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        BfVM::new(buffer.trim()).execute();
    }
}
