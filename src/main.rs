mod parser;

fn main() {
    let cs: Vec<char> = "hellooo, world!".chars().collect();

    let parser = parser::satisfy(|&ch| ch == 'h')
        .then(parser::satisfy(|&ch| ch == 'e'))
        .then(parser::choice(vec![
            parser::satisfy(|&ch| ch == 'k'),
            parser::satisfy(|&ch| ch == 'l'),
            parser::satisfy(|&ch| ch == 'j'),
        ]))
        .then(parser::satisfy(|&ch| ch == 'l'))
        .then(parser::many(parser::satisfy(|&ch| ch == 'o')))
        .into(String::from_iter);

    match parser.run(&cs) {
        Ok(a) => println!("succes: {:?}", a),
        Err(e) => match e {
            parser::ParseFailure::Eof(state) => println!("{:?}", state),
            parser::ParseFailure::UnexpectedChar(ch, state) => println!("{} {:?}", ch, state),
            parser::ParseFailure::OutOfOptions(state) => println!("{:?}", state),
        },
    }
}
