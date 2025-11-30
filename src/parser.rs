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
    fun: Box<dyn Fn(TextInputState<'a>) -> ParseResult<'a, T> + 'a>,
}

impl<'a, T> Parser<'a, T> {
    fn new<F>(f: F) -> Parser<'a, T>
    where
        F: Fn(TextInputState<'a>) -> ParseResult<'a, T> + 'a,
        T: 'a,
    {
        Parser { fun: Box::new(f) }
    }

    pub fn then<U, F>(self, f: F) -> Parser<'a, U>
    where
        F: Fn(T) -> Parser<'a, U> + 'a,
        T: 'a,
        U: 'a,
    {
        Parser::new(move |state| {
            let (value, new_state) = (self.fun)(state)?;
            let next_parser = f(value);
            (next_parser.fun)(new_state)
        })
    }

    pub fn run(self, input: &'a [char]) -> Result<T, ParseFailure<'a>> {
        let tis = TextInputState::new(input, 0);
        match (self.fun)(tis) {
            Ok((a, _)) => Ok(a),
            Err(e) => Err(e),
        }
    }
}

/* Basic Parsers */

// checks next char
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

pub fn choice<'a, T>(options: Vec<Parser<'a, T>>) -> Parser<'a, T>
where
    T: 'a,
{
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

pub fn many<'a, T>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>>
where
    T: 'a,
{
    Parser::new(move |mut state| {
        let mut acc = Vec::new();
        while let Ok((v, new_state)) = (parser.fun)(state) {
            acc.push(v);
            state = new_state;
        }

        Ok((acc, state))
    })
}

/*
pub fn many1()

pub fn pchar()

pub fn pstring()
*/
