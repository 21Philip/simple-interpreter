use crate::parser::{arith_eval, pprogram};
mod parser;

fn main() {
    let cs: Vec<char> = "(5-6) + 4 / 6 + -3".chars().collect();
    let program = match pprogram().run(&cs) {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    println!("{:?}", program);
    println!("{}", arith_eval(program));
}
