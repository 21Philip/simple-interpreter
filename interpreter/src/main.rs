use std::collections::HashMap;

use interpreter::eval::statement_eval;
use interpreter::parse::parse_ast;

fn main() {
    let input = "let a = 3+4 * 5 ; print a; print a-31; print -(3+4) / 7; print b";

    let ast = match parse_ast(input) {
        Ok(v) => {
            println!("{:?}", v);
            v
        }
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let mut vars: HashMap<String, i64> = HashMap::new();
    if let Err(e) = statement_eval(ast, &mut vars) {
        println!("{:?}", e)
    }
}
