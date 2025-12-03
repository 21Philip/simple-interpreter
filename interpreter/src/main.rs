use lib_parse::prelude::ParseFailure;
mod parser;

fn main() {
    let cs: Vec<char> = "hellooo,,,, world!".chars().collect();

    match parser::parser().run(&cs) {
        Ok(a) => println!("success: {:?}", a),
        Err(e) => match e {
            ParseFailure::Eof => println!("eof"),
            ParseFailure::UnexpectedChar(ch) => println!("unexpected '{}'", ch),
            ParseFailure::OutOfOptions => println!("out of options"),
            ParseFailure::MapError => println!("map error"),
        },
    }
}
