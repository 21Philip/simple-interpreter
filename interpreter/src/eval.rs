use crate::language::Aexpr;

pub fn arith_eval(expr: Aexpr) -> i64 {
    match expr {
        Aexpr::Add(a, b) => arith_eval(*a) + arith_eval(*b),
        Aexpr::Sub(a, b) => arith_eval(*a) - arith_eval(*b),
        Aexpr::Mul(a, b) => arith_eval(*a) * arith_eval(*b),
        Aexpr::Div(a, b) => arith_eval(*a) / arith_eval(*b),
        Aexpr::Negate(a) => -arith_eval(*a),
        Aexpr::Num(a) => a,
    }
}
