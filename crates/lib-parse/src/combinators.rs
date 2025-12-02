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

pub fn pi64() -> Parser<i64> {
    many1(&satisfy(|ch| ch.is_ascii_digit()))
        .map(String::from_iter)
        .map_fallible(|result| str::parse::<i64>(&result))
}
