mod parser;

fn main() {
    let cs: Vec<char> = "hello, world!".chars().collect();

    let parser = parser::satisfy(|&c| c == 'h').then(|_| parser::satisfy(|&c| c == 'e'));

    match parser.run(&cs) {
        Ok(a) => println!("succes: {}", a),
        Err(e) => match e {
            parser::ParseFailure::Eof(state) => println!("{:?}", state),
            parser::ParseFailure::UnexpectedChar(c, state) => println!("{} {:?}", c, state),
        },
    }
}
