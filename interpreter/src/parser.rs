use lib_parse::parsers::*;
use lib_parse::prelude::*;

/* ===== Language ===== */

#[derive(Debug)]
pub enum Aexpr {
    Num(i64),
    Add(Box<Aexpr>, Box<Aexpr>),
}

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

/* ===== "language name"-Parsing ===== */

fn padd() -> Parser<Aexpr> {
    pbinop(|| pchar('+'), p_a2, p_a1).map(|result| {
        let (a, b) = result;
        Aexpr::Add(Box::new(a), Box::new(b))
    })
}

fn p_a1() -> Parser<Aexpr> {
    choice([padd, p_a2].to_vec())
}

fn pnum() -> Parser<Aexpr> {
    pi64().map(Aexpr::Num)
}

fn p_a2() -> Parser<Aexpr> {
    choice([pnum, paexpr].to_vec())
}

pub fn paexpr() -> Parser<Aexpr> {
    choice([p_a1, p_a2].to_vec())
}
