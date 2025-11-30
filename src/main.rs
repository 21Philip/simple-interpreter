mod parser;

fn main() {
    let cs: Vec<char> = "hellooo, world!".chars().collect();

    let parser = parser::satisfy(|&ch| ch == 'h')
        .then(parser::satisfy(|&ch| ch == 'e'))
        .before(parser::choice(vec![
            parser::satisfy(|&ch| ch == 'k'),
            parser::satisfy(|&ch| ch == 'l'),
            parser::satisfy(|&ch| ch == 'j'),
        ]))
        .before(parser::satisfy(|&ch| ch == 'l'))
        .and(parser::many(parser::satisfy(|&ch| ch == 'o')))
        .into(|result| {
            let (ch, chs) = result;
            let s: String = std::iter::once(ch).chain(chs.iter().copied()).collect();
            s
        });

    match parser.run(&cs) {
        Ok(a) => println!("success: {:?}", a),
        Err(e) => match e {
            parser::ParseFailure::Eof(state) => println!("{:?}", state),
            parser::ParseFailure::UnexpectedChar(ch, state) => println!("{} {:?}", ch, state),
            parser::ParseFailure::OutOfOptions(state) => println!("{:?}", state),
        },
    }
}
