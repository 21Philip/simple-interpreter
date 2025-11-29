/*
enum Token {
    Semicolon
    Let
    Atomic(String),
    Op(String),
}

struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        input
            .split_whitespace()
            .map(|s|
                s.chars()
            );
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    tokenize(&input);
}

fn tokenize(text: &String) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    while let Some(c) = text.pop() {
        match c {

            'a'..='z'|'A'..='Z' => current.push(c),
            '+'|'-'|'*'|'/' =>
        }
    }
}

*/

mod parser;

fn main() {
    println!("{}", parser::test())
}
