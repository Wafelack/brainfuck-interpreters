use crate::scanner::tokens::Token;

pub struct Scanner {
    pub source: String,
    pub chars: Vec<char>,
    pub start: usize,
    pub current: usize,
    pub tokens: Vec<Token>,
}
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.clone(),
            chars: source.clone().chars().collect(),
            start: 0,
            current: 0,
            tokens: vec![],
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }
    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '-' => self.add_token(Token::Minus),
            '+' => self.add_token(Token::Plus),
            '[' => self.add_token(Token::LeftBrace),
            ']' => self.add_token(Token::RightBrace),
            '.' => self.add_token(Token::Dot),
            ',' => self.add_token(Token::Coma),
            '>' => self.add_token(Token::RightChevr),
            '<' => self.add_token(Token::LeftChevr),
            _ => (),
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.scan_token();
        }
        self.tokens.clone()
    }
}
