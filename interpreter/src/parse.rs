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

fn chain_left<T, F>(term: impl ParseClosure<T> + Copy, op: impl ParseClosure<F> + Copy) -> Parser<T>
where
    T: 'static,
    F: Fn(T, T) -> T + 'static,
{
    term()
        .and(move || many(move || spaces().then(op).ands(term)))
        .map(|(first, rest)| {
            rest.into_iter()
                .fold(first, |acc, (merge_fn, next)| merge_fn(acc, next))
        })
}

fn binop<T: 'static>(ch: char, constructor: fn(Box<T>, Box<T>) -> T) -> Parser<impl Fn(T, T) -> T> {
    pchar(ch).map(move |_| move |a, b| constructor(Box::new(a), Box::new(b)))
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

fn paddsub() -> Parser<Aexpr> {
    chain_left(p_ae2, || {
        choice([|| binop('+', Aexpr::Add), || binop('-', Aexpr::Sub)].to_vec())
    })
}

fn p_ae1() -> Parser<Aexpr> {
    choice([paddsub, p_ae2].to_vec())
}

// Precedence 2:

fn pmuldiv() -> Parser<Aexpr> {
    chain_left(p_ae3, || {
        choice([|| binop('*', Aexpr::Mul), || binop('/', Aexpr::Div)].to_vec())
    })
}

fn p_ae2() -> Parser<Aexpr> {
    choice([pmuldiv, p_ae3].to_vec())
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
    p_ae1()
}

/* === Statements === */

// Precedence 1:

fn psequence() -> Parser<Statement> {
    fn merge_sequence(a: Statement, b: Statement) -> Statement {
        match a {
            Statement::Sequence(mut queue) => {
                queue.push_back(b);
                Statement::Sequence(queue)
            }
            _ => Statement::Sequence(VecDeque::from([a, b])),
        }
    }

    chain_left(p_stmnt2, || pchar(';').map(|_| merge_sequence))
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
    p_stmnt1()
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
