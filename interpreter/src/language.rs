use std::collections::VecDeque;

#[derive(Debug)]
pub enum Atomic {
    Int(i64),
    //Bool(bool),
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
pub enum Bexpr {
    And(Box<Bexpr>, Box<Bexpr>),
    Or(Box<Bexpr>, Box<Bexpr>),
    Not(Box<Bexpr>),
}

#[derive(Debug)]
pub enum Statement {
    // Precedence 1
    Sequence(VecDeque<Statement>),
    // Precedence 2
    Assign(String, Aexpr),
    Print(Aexpr),
}
