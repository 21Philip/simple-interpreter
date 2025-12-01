use lib_parse::combinators::*;
use lib_parse::prelude::*;

fn main() {
    let cs: Vec<char> = "hellooo,,,, world!".chars().collect();

    let subparser = satisfy(|_| true).and(&satisfy(|_| true));
    let subparser2 = pchar('e');

    let parser = satisfy(|&ch| ch == 'h')
        .then(&subparser2)
        .then(&subparser)
        .then(&satisfy(|&ch| ch == 'o'))
        .then(&subparser)
        .then(&many1(&satisfy(|&ch| ch == ',')))
        .map(String::from_iter);

    match parser.run(&cs) {
        Ok(a) => println!("success: {:?}", a),
        Err(e) => match e {
            ParseFailure::Eof(state) => println!("{:?}", state),
            ParseFailure::UnexpectedChar(ch, state) => println!("{} {:?}", ch, state),
            ParseFailure::OutOfOptions(state) => println!("{:?}", state),
        },
    }
}
