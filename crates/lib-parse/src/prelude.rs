use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
struct TextInputState<'a> {
    text: &'a [char],
    position: usize,
}

impl<'a> TextInputState<'a> {
    fn new(input: &'a [char], pos: usize) -> TextInputState<'a> {
        TextInputState {
            text: input,
            position: pos,
        }
    }

    fn next_char(self) -> Option<(char, TextInputState<'a>)> {
        let opt = self.text.get(self.position).copied();
        opt.map(|ch| (ch, TextInputState::new(self.text, self.position + 1)))
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseFailure {
    Eof,
    UnexpectedChar(char),
    OutOfOptions,
    MapError,
}

type ParseResult<'a, T> = Result<(T, TextInputState<'a>), ParseFailure>;

pub struct Parser<T> {
    fun: Rc<dyn for<'a> Fn(TextInputState<'a>) -> ParseResult<'a, T>>,
}

impl<T> Clone for Parser<T> {
    fn clone(&self) -> Self {
        Parser {
            fun: Rc::clone(&self.fun),
        }
    }
}

impl<T: 'static> Parser<T> {
    fn new<F>(f: F) -> Parser<T>
    where
        F: for<'a> Fn(TextInputState<'a>) -> ParseResult<'a, T> + 'static,
    {
        Parser { fun: Rc::new(f) }
    }

    // Discards result from previous parser
    pub fn then<U: 'static>(self, next: &Parser<U>) -> Parser<U> {
        let next = next.clone();

        Parser::new(move |state| {
            let (_, new_state) = (self.fun)(state)?;
            (next.fun)(new_state)
        })
    }

    // Discards result from next parser
    pub fn before<U: 'static>(self, next: &Parser<U>) -> Parser<T> {
        let next = next.clone();

        Parser::new(move |state| {
            let (value, new_state) = (self.fun)(state)?;
            let (_, new_state) = (next.fun)(new_state)?;
            Ok((value, new_state))
        })
    }

    // Keeps results from both prevoius and next parser
    pub fn and<U: 'static>(self, next: &Parser<U>) -> Parser<(T, U)> {
        let next = next.clone();

        Parser::new(move |state| {
            let (value1, new_state) = (self.fun)(state)?;
            let (value2, new_state) = (next.fun)(new_state)?;
            Ok(((value1, value2), new_state))
        })
    }

    // Maps the result of a parser by a given function
    pub fn map<U, F>(self, f: F) -> Parser<U>
    where
        F: Fn(T) -> U + 'static,
        U: 'static,
    {
        Parser::new(move |state| {
            let (value, new_state) = (self.fun)(state)?;
            Ok((f(value), new_state))
        })
    }

    // Maps the result of a parser by a function that might fail
    pub fn map_fallible<U, E, F>(self, f: F) -> Parser<U>
    where
        F: Fn(T) -> Result<U, E> + 'static,
        E: 'static,
        U: 'static,
    {
        Parser::new(move |state| {
            let (value, new_state) = (self.fun)(state)?;
            match f(value) {
                Ok(v) => Ok((v, new_state)),
                Err(_) => Err(ParseFailure::MapError),
            }
        })
    }

    // Runs parser on given input and returns result
    pub fn run(self, input: &[char]) -> Result<T, ParseFailure> {
        let tis = TextInputState::new(input, 0);
        match (self.fun)(tis) {
            Ok((value, _)) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

/* Combinators + satisfy */

pub fn satisfy<F>(predicate: F) -> Parser<char>
where
    F: Fn(char) -> bool + 'static,
{
    Parser::new(move |state| match state.next_char() {
        None => Err(ParseFailure::Eof),
        Some((ch, new_state)) if predicate(ch) => Ok((ch, new_state)),
        Some((ch, _)) => Err(ParseFailure::UnexpectedChar(ch)),
    })
}

pub fn choice<T: 'static>(options: &[&Parser<T>]) -> Parser<T> {
    let options: Vec<Parser<T>> = options.iter().map(|p| (*p).clone()).collect();

    Parser::new(move |state| {
        for parser in &options {
            match (parser.fun)(state) {
                Ok(v) => return Ok(v),
                Err(_) => continue,
            }
        }
        Err(ParseFailure::OutOfOptions)
    })
}

pub fn sequence<T: 'static>(parsers: &[&Parser<T>]) -> Parser<Vec<T>> {
    let parsers: Vec<Parser<T>> = parsers.iter().map(|p| (*p).clone()).collect();

    Parser::new(move |mut state| {
        let mut acc = Vec::new();

        for parser in &parsers {
            let (value, new_state) = (parser.fun)(state)?;
            acc.push(value);
            state = new_state;
        }

        Ok((acc, state))
    })
}

pub fn many<T: 'static>(parser: &Parser<T>) -> Parser<Vec<T>> {
    let parser = parser.clone();

    Parser::new(move |mut state| {
        let mut acc = Vec::new();
        while let Ok((value, new_state)) = (parser.fun)(state) {
            acc.push(value);
            state = new_state;
        }
        Ok((acc, state))
    })
}

pub fn many1<T: 'static>(parser: &Parser<T>) -> Parser<Vec<T>> {
    let parser = parser.clone();

    Parser::new(move |state| {
        let (first, state) = (parser.fun)(state)?;
        let (mut rest, state) = (many(&parser).fun)(state)?;
        rest.insert(0, first);
        Ok((rest, state))
    })
}

pub fn optional<T: 'static>(parser: &Parser<T>) -> Parser<Option<T>> {
    let parser = parser.clone();

    Parser::new(move |state| match (parser.fun)(state) {
        Ok((value, new_state)) => Ok((Some(value), new_state)),
        Err(_) => Ok((None, state)),
    })
}

pub fn or_else<T: 'static>(p1: &Parser<T>, p2: &Parser<T>) -> Parser<T> {
    choice(&[p1, p2])
}

pub fn between<T, U, S>(left: &Parser<T>, right: &Parser<U>, mid: &Parser<S>) -> Parser<S>
where
    T: 'static,
    U: 'static,
    S: 'static,
{
    let left = left.clone();
    left.then(mid).before(right)
}

pub fn lazy<T, F>(f: F) -> Parser<T>
where
    F: Fn() -> Parser<T> + 'static,
    T: 'static,
{
    Parser::new(move |state| {
        let parser = f();
        (parser.fun)(state)
    })
}
