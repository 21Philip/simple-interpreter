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

    fn next_char(self) -> (Option<char>, TextInputState<'a>) {
        let opt = self.text.get(self.position).copied();
        match opt {
            Some(c) => (Some(c), TextInputState::new(self.text, self.position + 1)),
            _ => (None, TextInputState::new(self.text, self.position)),
        }
    }
}

enum ParseFailure<'a> {
    EOF(TextInputState<'a>),
    UnexpectedChar(String, TextInputState<'a>),
}

type ParseResult<'a, T> = Result<(T, TextInputState<'a>), ParseFailure<'a>>;

struct Parser<'a, T> {
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
}

/* Basic Parsers */

// checks next char
pub fn satisfy<'a, F>(predicate: F) -> Parser<'a, char>
where
    F: Fn(&char) -> bool + 'a,
{
    Parser::new(move |state| {
        let (c, new_state) = state.next_char();
        match c {
            None => Err(ParseFailure::EOF(new_state)),
            Some(c) if predicate(&c) => Ok((c, new_state)),
            Some(c) => Err(ParseFailure::UnexpectedChar(
                format!("got unexpected char {c} at position {}", new_state.position),
                new_state,
            )),
        }
    })
}

/*
pub fn choice()

pub fn many()

pub fn many1()

pub fn pchar()

pub fn pstring()
*/
