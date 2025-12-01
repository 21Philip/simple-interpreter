use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct TextInputState<'a> {
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

    fn next_char(self) -> (Option<char>, TextInputState<'a>) {
        let opt = self.text.get(self.position).copied();
        match opt {
            Some(ch) => (Some(ch), TextInputState::new(self.text, self.position + 1)),
            _ => (None, TextInputState::new(self.text, self.position)),
        }
    }
}

pub enum ParseFailure<'a> {
    Eof(TextInputState<'a>),
    UnexpectedChar(char, TextInputState<'a>),
    OutOfOptions(TextInputState<'a>),
}

type ParseResult<'a, T> = Result<(T, TextInputState<'a>), ParseFailure<'a>>;

pub struct Parser<'a, T> {
    fun: Rc<dyn Fn(TextInputState<'a>) -> ParseResult<'a, T> + 'a>,
}

impl<'a, T> Clone for Parser<'a, T> {
    fn clone(&self) -> Self {
        Parser {
            fun: Rc::clone(&self.fun),
        }
    }
}

impl<'a, T> Parser<'a, T> {
    fn new<F>(f: F) -> Parser<'a, T>
    where
        F: Fn(TextInputState<'a>) -> ParseResult<'a, T> + 'a,
        T: 'a,
    {
        Parser { fun: Rc::new(f) }
    }

    // Discards result from previous parser
    pub fn then<U>(self, next: &Parser<'a, U>) -> Parser<'a, U>
    where
        T: 'a,
        U: 'a,
    {
        let next = next.clone();
        Parser::new(move |state| {
            let (_, new_state) = (self.fun)(state)?;
            (next.fun)(new_state)
        })
    }

    // Discards result from next parser
    pub fn before<U>(self, next: &Parser<'a, U>) -> Parser<'a, T>
    where
        T: 'a,
        U: 'a,
    {
        let next = next.clone();
        Parser::new(move |state| {
            let (value, new_state) = (self.fun)(state)?;
            let (_, new_state) = (next.fun)(new_state)?;
            Ok((value, new_state))
        })
    }

    // Keeps results from both prevoius and next parser
    pub fn and<U>(self, next: &Parser<'a, U>) -> Parser<'a, (T, U)>
    where
        T: 'a,
        U: 'a,
    {
        let next = next.clone();
        Parser::new(move |state| {
            let (value1, new_state) = (self.fun)(state)?;
            let (value2, new_state) = (next.fun)(new_state)?;
            Ok(((value1, value2), new_state))
        })
    }

    // Maps the result of a parser by a given function
    pub fn map<U, F>(self, f: F) -> Parser<'a, U>
    where
        F: Fn(T) -> U + 'a,
        T: 'a,
        U: 'a,
    {
        Parser::new(move |state| {
            let (value, new_state) = (self.fun)(state)?;
            Ok((f(value), new_state))
        })
    }

    // Runs parser on given input and returns result
    pub fn run(self, input: &'a [char]) -> Result<T, ParseFailure<'a>> {
        let tis = TextInputState::new(input, 0);
        match (self.fun)(tis) {
            Ok((value, _)) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

/* Base combinators:
 * These builders can only be implemented through private internal logic.
 * They are the building blocks for all other parsers.
 */

pub fn satisfy<'a, F>(predicate: F) -> Parser<'a, char>
where
    F: Fn(&char) -> bool + 'a,
{
    Parser::new(move |state| {
        let (opt, new_state) = state.next_char();
        match opt {
            None => Err(ParseFailure::Eof(new_state)),
            Some(ch) if predicate(&ch) => Ok((ch, new_state)),
            Some(ch) => Err(ParseFailure::UnexpectedChar(ch, new_state)),
        }
    })
}

pub fn choice<'a, T>(options: &[Parser<'a, T>]) -> Parser<'a, T>
where
    T: 'a,
{
    let options = options.to_vec();
    Parser::new(move |state| {
        for parser in &options {
            match (parser.fun)(state) {
                Ok(v) => return Ok(v),
                Err(_) => continue,
            }
        }
        Err(ParseFailure::OutOfOptions(state))
    })
}

pub fn sequence<'a, T>(parsers: &[Parser<'a, T>]) -> Parser<'a, Vec<T>>
where
    T: 'a,
{
    let parsers = parsers.to_vec();
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

pub fn many<'a, T>(parser: &Parser<'a, T>) -> Parser<'a, Vec<T>>
where
    T: 'a,
{
    let parser = parser.clone();

    Parser::new(move |mut state| {
        let mut acc = Vec::new();
        while let Ok((v, new_state)) = (parser.fun)(state) {
            acc.push(v);
            state = new_state;
        }
        Ok((acc, state))
    })
}
