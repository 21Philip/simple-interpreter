struct TextInputState<'a> {
    text: &'a [char],
    position: i32,
}

struct ParseError<'a> {
    state: TextInputState<'a>,
    error: String,
}

type ParseResult<'a, T> = Result<(T, TextInputState<'a>), ParseError<'a>>;

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

    fn then<U, F>(self, f: F) -> Parser<'a, U>
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
