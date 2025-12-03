use lib_parse::parsers::*;
use lib_parse::prelude::*;

enum AExpr {
    Num(i64),
    Add(Box<AExpr>, Box<AExpr>),
}

pub fn parser() -> Parser<char> {
    pchar('h')
}
