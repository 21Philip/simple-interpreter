use crate::prelude::*;

/* Convenience combinators:
 * Basic parsers to save the user from implementing trivial logic
 * themselves. Are all combinations of the essiential builders.
 */

pub fn many1<T: 'static>(parser: &Parser<T>) -> Parser<Vec<T>> {
    parser.clone().and(&many(parser)).map(|result| {
        let (first, rest) = result;
        std::iter::once(first).chain(rest).collect()
    })
}

pub fn pchar(ch: char) -> Parser<char> {
    satisfy(move |&c| c == ch)
}

pub fn pstring(s: &str) -> Parser<String> {
    let parsers: Vec<Parser<char>> = s.chars().map(pchar).collect();
    sequence(&parsers).map(String::from_iter)
}
