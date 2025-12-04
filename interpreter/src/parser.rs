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

trait IgnoreWhitespace<T> {
    fn thens<F, U>(self, next: F) -> Parser<U>
    where
        F: Fn() -> Parser<U> + 'static,
        U: 'static;
    fn befores<F, U>(self, next: F) -> Parser<T>
    where
        F: Fn() -> Parser<U> + 'static,
        U: 'static;
    fn ands<F, U>(self, next: F) -> Parser<(T, U)>
    where
        F: Fn() -> Parser<U> + 'static,
        U: 'static;
}

impl<T: 'static> IgnoreWhitespace<T> for Parser<T> {
    fn thens<F, U>(self, next: F) -> Parser<U>
    where
        F: Fn() -> Parser<U> + 'static,
        U: 'static,
    {
        self.then(spaces).then(next)
    }

    fn befores<F, U>(self, next: F) -> Parser<T>
    where
        F: Fn() -> Parser<U> + 'static,
        U: 'static,
    {
        self.before(spaces).before(next)
    }

    fn ands<F, U>(self, next: F) -> Parser<(T, U)>
    where
        F: Fn() -> Parser<U> + 'static,
        U: 'static,
    {
        self.before(spaces).and(next)
    }
}

fn pbinop<F, G, H, T, U, S>(op: F, a: G, b: H) -> Parser<(U, S)>
where
    F: Fn() -> Parser<T> + 'static,
    G: Fn() -> Parser<U> + 'static,
    H: Fn() -> Parser<S> + 'static,
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
