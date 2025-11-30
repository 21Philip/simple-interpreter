use crate::prelude::*;

/* Convinience combinators:
 * Basic parsers to save the user from implementing trivial logic
 * themselves. Are all combinations of the essiential builders.
 * Does not take ownership.
 */

pub fn many1<'a, T>(parser: &Parser<'a, T>) -> Parser<'a, Vec<T>>
where
    T: 'a,
{
    parser.clone().and(&many(parser.clone())).map(|result| {
        let (first, rest) = result;
        std::iter::once(first).chain(rest).collect()
    })
}

//pub fn pchar()

/*
pub fn pstring()
*/

