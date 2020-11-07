use crate::scanner::tokens::Token;

pub fn parse(toks: Vec<Token>) -> String {
    let mut code =
        String::from("#include <stdio.h>\n\nint main(void) {\nunsigned char ptr[1024];\n\n");

    for tok in toks {
        match tok {
            Token::LeftChevr => code.push_str("ptr--;\n"),
            Token::RightChevr => code.push_str("ptr++;\n"),
            Token::Plus => code.push_str("++(*ptr);\n"),
            Token::Minus => code.push_str("--(*ptr);\n"),
            Token::LeftBrace => code.push_str("while (*ptr) {\n"),
            Token::RightBrace => code.push_str("}\n"),
            Token::Dot => code.push_str("putchar(*ptr);\n"),
            Token::Coma => code.push_str("(*ptr) = getchar();\n"),
        }
    }
    code.push_str("}");
    code
}
