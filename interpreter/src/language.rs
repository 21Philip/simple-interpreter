use std::collections::VecDeque;

#[derive(Debug)]
pub enum Atomic {
    Int(i64),
    Identifier(String),
}

#[derive(Debug)]
pub enum Aexpr {
    // Precedence 1
    Add(Box<Aexpr>, Box<Aexpr>),
    Sub(Box<Aexpr>, Box<Aexpr>),
    // Precedence 2
    Mul(Box<Aexpr>, Box<Aexpr>),
    Div(Box<Aexpr>, Box<Aexpr>),
    // Precedence 3
    Negate(Box<Aexpr>),
    Num(Atomic),
}

#[derive(Debug)]
pub enum Statement {
    Sequence(VecDeque<Statement>),
    Assign(String, Aexpr),
    Print(Aexpr),
}
