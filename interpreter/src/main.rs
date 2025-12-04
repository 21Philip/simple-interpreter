use crate::parser::paexpr;
use lib_parse::prelude::*;
mod parser;

fn main() {
    let cs: Vec<char> = "3 + 2".chars().collect();

    match paexpr().run(&cs) {
        Ok(a) => println!("success: {:?}", a),
        Err(e) => match e {
            ParseFailure::Eof => println!("eof"),
            ParseFailure::UnexpectedChar(ch) => println!("unexpected '{}'", ch),
            ParseFailure::OutOfOptions => println!("out of options"),
            ParseFailure::MapError => println!("map error"),
        },
    }
}
