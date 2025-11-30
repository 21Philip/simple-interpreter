mod parser;

fn main() {
    let cs: Vec<char> = "hellooo, world!".chars().collect();

    let subparser = parser::satisfy(|_| true).and(&parser::satisfy(|_| true));

    let parser = parser::satisfy(|&ch| ch == 'h')
        .then(&parser::satisfy(|&ch| ch == 'e'))
        .then(&subparser)
        .then(&parser::satisfy(|&ch| ch == 'o'))
        .then(&subparser);
    //.then(&many1(parser))

    match parser.run(&cs) {
        Ok(a) => println!("success: {:?}", a),
        Err(e) => match e {
            parser::ParseFailure::Eof(state) => println!("{:?}", state),
            parser::ParseFailure::UnexpectedChar(ch, state) => println!("{} {:?}", ch, state),
            parser::ParseFailure::OutOfOptions(state) => println!("{:?}", state),
        },
    }
}
