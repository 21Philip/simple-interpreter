use lib_parse::combinators::*;
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
            ParseFailure::Eof(state) => println!("{:?}", state),
            ParseFailure::UnexpectedChar(ch, state) => println!("{} {:?}", ch, state),
            ParseFailure::OutOfOptions(state) => println!("{:?}", state),
        },
    }
}
