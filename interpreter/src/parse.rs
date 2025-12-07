use std::collections::VecDeque;

use crate::language::{Aexpr, Atomic, Statement};
use lib_parse::parsers::{pchar, pi64, pstring};
use lib_parse::prelude::*;

/* ===== Generic Parsers & Helpers ===== */

fn pwhitespace() -> Parser<char> {
    satisfy(|ch| ch.is_whitespace())
}

fn spaces() -> Parser<Vec<char>> {
    many(pwhitespace)
}

fn spaces1() -> Parser<Vec<char>> {
    many1(pwhitespace)
}

trait IgnoreWhitespace<T: 'static, U: 'static> {
    fn thens(self, next: impl ParseClosure<U>) -> Parser<U>;
    fn befores(self, next: impl ParseClosure<U>) -> Parser<T>;
    fn ands(self, next: impl ParseClosure<U>) -> Parser<(T, U)>;
}

impl<T: 'static, U: 'static> IgnoreWhitespace<T, U> for Parser<T> {
    fn thens(self, next: impl ParseClosure<U>) -> Parser<U> {
        self.then(spaces).then(next)
    }

    fn befores(self, next: impl ParseClosure<U>) -> Parser<T> {
        self.before(spaces).before(next)
    }

    fn ands(self, next: impl ParseClosure<U>) -> Parser<(T, U)> {
        self.before(spaces).and(next)
    }
}

fn pbinop<T, U, S>(
    op: impl ParseClosure<T>,
    a: impl ParseClosure<U>,
    b: impl ParseClosure<S>,
) -> Parser<(U, S)>
where
    T: 'static,
    U: 'static,
    S: 'static,
{
    a().befores(op).ands(b)
}

fn between<T, U, S>(
    left: impl ParseClosure<T>,
    right: impl ParseClosure<U>,
    mid: impl ParseClosure<S>,
) -> Parser<S>
where
    T: 'static,
    U: 'static,
    S: 'static,
{
    left().thens(mid).befores(right)
}

fn pid() -> Parser<String> {
    satisfy(|ch| ch.is_alphabetic())
        .and(|| many(|| satisfy(|ch| ch.is_alphanumeric())))
        .map(|(first, mut rest)| {
            rest.insert(0, first);
            String::from_iter(rest)
        })
}

/* ===== "language name"-Parsing ===== */

/* === Atomic Values === */

fn pint() -> Parser<Atomic> {
    pi64().map(Atomic::Int)
}

fn pidentifier() -> Parser<Atomic> {
    pid().map(Atomic::Identifier)
}

fn patomic() -> Parser<Atomic> {
    choice([pint, pidentifier].to_vec())
}

/* === Arithmetic Expressions === */

// Precedence 1:

fn padd() -> Parser<Aexpr> {
    pbinop(|| pchar('+'), p_ae2, p_ae1).map(|(a, b)| Aexpr::Add(Box::new(a), Box::new(b)))
}

fn psub() -> Parser<Aexpr> {
    pbinop(|| pchar('-'), p_ae2, p_ae1).map(|(a, b)| Aexpr::Sub(Box::new(a), Box::new(b)))
}

fn p_ae1() -> Parser<Aexpr> {
    choice([padd, psub, p_ae2].to_vec())
}

// Precedence 2:

fn pmul() -> Parser<Aexpr> {
    pbinop(|| pchar('*'), p_ae3, p_ae2).map(|(a, b)| Aexpr::Mul(Box::new(a), Box::new(b)))
}

fn pdiv() -> Parser<Aexpr> {
    pbinop(|| pchar('/'), p_ae3, p_ae2).map(|(a, b)| Aexpr::Div(Box::new(a), Box::new(b)))
}

fn p_ae2() -> Parser<Aexpr> {
    choice([pmul, pdiv, p_ae3].to_vec())
}

// Precedence 3:

fn pnegate() -> Parser<Aexpr> {
    pchar('-')
        .then(p_ae3) // maybe thens?
        .map(|result| Aexpr::Negate(Box::new(result)))
}

fn pparentheses() -> Parser<Aexpr> {
    // 'mid' argument must be lowest precedence level
    between(|| pchar('('), || pchar(')'), p_ae1)
}

fn pnum() -> Parser<Aexpr> {
    patomic().map(Aexpr::Num)
}

fn p_ae3() -> Parser<Aexpr> {
    // highest precedence does not call up.
    // pnum might parse '-' so order between
    // pnegate and pnum matters for AST (eval is same?).
    choice([pnegate, pparentheses, pnum].to_vec())
}

// Full arithmetic expression

fn paexpr() -> Parser<Aexpr> {
    choice([p_ae1, p_ae2, p_ae3].to_vec())
}

/* === Statements === */

// Precedence 1:

fn psequence() -> Parser<Statement> {
    pbinop(|| pchar(';'), p_stmnt2, p_stmnt1).map(|(a, b)| match b {
        Statement::Sequence(mut queue) => {
            queue.push_front(a);
            Statement::Sequence(queue)
        }
        _ => Statement::Sequence(VecDeque::from([a, b])),
    })
}

fn p_stmnt1() -> Parser<Statement> {
    choice([psequence, p_stmnt2].to_vec())
}

// Precedence 2:

fn passign() -> Parser<Statement> {
    pstring("let")
        .then(spaces1)
        .then(pid)
        .befores(|| pchar('='))
        .ands(paexpr)
        .map(|(id, value)| Statement::Assign(id, value))
}

fn pprint() -> Parser<Statement> {
    pstring("print")
        .then(spaces1)
        .then(paexpr)
        .map(Statement::Print)
}

fn p_stmnt2() -> Parser<Statement> {
    choice([passign, pprint].to_vec())
}

// Full statement

fn pstatement() -> Parser<Statement> {
    choice([p_stmnt1, p_stmnt2].to_vec())
}

/* === Full AST parser === */

pub fn parse_ast(input: &str) -> Result<Statement, ParseFailure> {
    let chars: Vec<char> = input.chars().collect();
    spaces()
        .then(pstatement)
        .befores(|| optional(|| pchar(';')))
        .befores(eof)
        .run(&chars)
}
