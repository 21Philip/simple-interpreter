use lib_parse::parsers::*;
use lib_parse::prelude::*;

fn main() {
    let cs: Vec<char> = "hellooo,,,, world!".chars().collect();

    let parser = pchar('h')
        .then(&pchar('e'))
        .then(&many1(&pchar('l')))
        .then(&many(&pchar('o')))
        .then(&pstring(",,,,"));

    match parser.run(&cs) {
        Ok(a) => println!("success: {:?}", a),
        Err(e) => match e {
            ParseFailure::Eof => println!("eof"),
            ParseFailure::UnexpectedChar(ch) => println!("uexpected '{}'", ch),
            ParseFailure::OutOfOptions => println!("out of options"),
            ParseFailure::MapError => println!("map error"),
        },
    }
}
