use crate::language::{Aexpr, Atomic};

pub fn atomic_eval(atom: Atomic) -> i64 {
    match atom {
        Atomic::Int(i) => i,
        Atomic::Identifier(_) => 0, // to be implemented
    }
}

pub fn arith_eval(expr: Aexpr) -> i64 {
    match expr {
        Aexpr::Add(a, b) => arith_eval(*a) + arith_eval(*b),
        Aexpr::Sub(a, b) => arith_eval(*a) - arith_eval(*b),
        Aexpr::Mul(a, b) => arith_eval(*a) * arith_eval(*b),
        Aexpr::Div(a, b) => arith_eval(*a) / arith_eval(*b),
        Aexpr::Negate(a) => -arith_eval(*a),
        Aexpr::Num(a) => atomic_eval(a),
    }
}
