use crate::prelude::*;

/* Convenience combinators:
 * Basic parsers to save the user from implementing trivial logic
 * themselves. Are all combinations of the essiential builders.
 */

pub fn many1<'a, T>(parser: &Parser<'a, T>) -> Parser<'a, Vec<T>>
where
    T: 'a,
{
    parser.clone().and(&many(parser)).map(|result| {
        let (first, rest) = result;
        std::iter::once(first).chain(rest).collect()
    })
}

pub fn pchar<'a>(ch: char) -> Parser<'a, char> {
    satisfy(move |&c| c == ch)
}

pub fn pstring<'a>(s: &str) -> Parser<'a, String> {
    let parsers: Vec<Parser<'a, char>> = s.chars().map(pchar).collect();
    sequence(&parsers).map(String::from_iter)
}
