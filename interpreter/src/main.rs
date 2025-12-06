use interpreter::eval::arith_eval;
use interpreter::parse::parse_ast;

fn main() {
    let program = match parse_ast("(5-6) + 4 / 6 + -3") {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    println!("{:?}", program);
    println!("{}", arith_eval(program));
}
