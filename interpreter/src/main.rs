use crate::parser::pprogram;
use lib_parse::prelude::*;
mod parser;

fn main() {
    let cs: Vec<char> = "(5-6) + 4 / 6 + -3".chars().collect();

    match pprogram().run(&cs) {
        Ok(a) => println!("success: {:?}", a),
        Err(e) => match e {
            ParseFailure::Eof => println!("eof"),
            ParseFailure::UnexpectedChar(ch) => println!("unexpected '{}'", ch),
            ParseFailure::OutOfOptions => println!("out of options"),
            ParseFailure::MapError => println!("map error"),
        },
    }
}
