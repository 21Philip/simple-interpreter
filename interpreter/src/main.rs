use std::collections::HashMap;

use interpreter::eval::statement_eval;
use interpreter::parse::parse_ast;

fn main() {
    let input = "let x = 1+2-3; print x;";

    let ast = match parse_ast(input) {
        Ok(ast) => ast,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    println!("AST:\n{:?}\nOutput:", ast);

    let mut vars: HashMap<String, i64> = HashMap::new();
    if let Err(e) = statement_eval(ast, &mut vars) {
        println!("{:?}", e)
    }
}
