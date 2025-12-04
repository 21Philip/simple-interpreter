use lib_parse::parsers::*;
use lib_parse::prelude::*;

/* ===== Language ===== */

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
    Num(i64),
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

pub fn between<T, U, S>(
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

/* ===== "language name"-Parsing ===== */

/* === Arithmetic Expressions === */

// Level 1

fn padd() -> Parser<Aexpr> {
    pbinop(|| pchar('+'), p_ae2, p_ae1).map(|result| {
        let (a, b) = result;
        Aexpr::Add(Box::new(a), Box::new(b))
    })
}

fn psub() -> Parser<Aexpr> {
    pbinop(|| pchar('-'), p_ae2, p_ae1).map(|result| {
        let (a, b) = result;
        Aexpr::Sub(Box::new(a), Box::new(b))
    })
}

fn p_ae1() -> Parser<Aexpr> {
    choice([padd, psub, p_ae2].to_vec())
}

// Level 2

fn pmul() -> Parser<Aexpr> {
    pbinop(|| pchar('*'), p_ae3, p_ae2).map(|result| {
        let (a, b) = result;
        Aexpr::Mul(Box::new(a), Box::new(b))
    })
}

fn pdiv() -> Parser<Aexpr> {
    pbinop(|| pchar('/'), p_ae3, p_ae2).map(|result| {
        let (a, b) = result;
        Aexpr::Div(Box::new(a), Box::new(b))
    })
}

fn p_ae2() -> Parser<Aexpr> {
    choice([pmul, pdiv, p_ae3].to_vec())
}

// Level 3

fn pnegate() -> Parser<Aexpr> {
    pchar('-')
        .then(p_ae3) // maybe thens?
        .map(|result| Aexpr::Negate(Box::new(result)))
}

fn pparentheses() -> Parser<Aexpr> {
    // mid argument must be lowest precedence level
    between(|| pchar('('), || pchar(')'), p_ae1)
}

fn pnum() -> Parser<Aexpr> {
    pi64().map(Aexpr::Num)
}

fn p_ae3() -> Parser<Aexpr> {
    // highest level does not call up.
    // pnum parses '-' so order between
    // pnegate and pnum matters.
    choice([pnegate, pparentheses, pnum].to_vec())
}

// Full arithmetic expression

fn paexpr() -> Parser<Aexpr> {
    choice([p_ae1, p_ae2, p_ae3].to_vec())
}

/* ===== Program ===== */

pub fn pprogram() -> Parser<Aexpr> {
    spaces().then(paexpr).befores(eof)
}
