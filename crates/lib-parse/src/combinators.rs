use crate::prelude::*;

/* Parsers:
 * Basic parsers to save the user from implementing trivial logic
 * themselves. Are all combinations of the essiential builders.
 */

pub fn pchar(ch: char) -> Parser<char> {
    satisfy(move |c| c == ch)
}

pub fn pstring(s: &str) -> Parser<String> {
    let parsers: Vec<Parser<char>> = s.chars().map(pchar).collect();
    let borrowed: Vec<&Parser<char>> = parsers.iter().collect();
    sequence(&borrowed).map(String::from_iter)
}

pub fn pdigit() -> Parser<char> {
    satisfy(|ch| ch.is_ascii_digit())
}

pub fn pi64() -> Parser<i64> {
    optional(&pchar('-'))
        .and(&many1(&pdigit()))
        .map_fallible(|result| {
            let (sign, digits) = result;
            let number = String::from_iter(digits);
            str::parse::<i64>(&number).map(|n| n * if sign.is_some() { -1 } else { 1 })
        })
}
