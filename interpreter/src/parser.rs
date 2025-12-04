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

    fn ands<F, U>(self, next: &Parser<U>) -> Parser<(T, U)>
    where
        F: Fn() -> Parser<U> + 'static,
        U: 'static,
    {
        self.before(spaces()).and(next)
    }
}

fn pbinop<T, U, S>(op: &Parser<T>, a: &Parser<U>, b: &Parser<S>) -> Parser<(U, S)>
where
    T: 'static,
    U: 'static,
    S: 'static,
{
    a.clone().befores(op).ands(b)
}

/* ===== "language name"-Parsing ===== */

fn padd() -> Parser<Aexpr> {
    pbinop(&lazy(|| pchar('+')), &lazy(p_a2), &lazy(p_a1)).map(|result| {
        let (a, b) = result;
        Aexpr::Add(Box::new(a), Box::new(b))
    })
}

fn p_a1() -> Parser<Aexpr> {
    choice(&[&lazy(padd), &lazy(p_a2)])
}

fn pnum() -> Parser<Aexpr> {
    pi64().map(Aexpr::Num)
}

fn p_a2() -> Parser<Aexpr> {
    choice(&[&lazy(pnum), &lazy(paexpr)])
}

pub fn paexpr() -> Parser<Aexpr> {
    choice(&[&lazy(p_a1), &lazy(p_a2)])
}
