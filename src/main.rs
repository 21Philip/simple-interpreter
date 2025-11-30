mod parser;

fn main() {
    let s = "hello, world!".chars().collect();

    match parser::satisfy(|c| c == 'h').run(&s) {
        Ok(a) => println!("succes: {}", a),
        Err(e) => println!("error: {}", e),
    }
}
