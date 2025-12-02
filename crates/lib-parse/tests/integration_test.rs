use lib_parse::prelude::*;

fn run<'a, T>(p: Parser<'a, T>, input: &str) -> Result<T, ParseFailure<'a>> {
    p.clone().run(&input.chars().collect::<Vec<_>>())
}

trait ParserTestExt<'a, T> {
    fn run_str(&self, s: &'a str) -> Result<T, ParseFailure<'a>>;
}

impl<'a, T> ParserTestExt<'a, T> for Parser<'a, T> {
    fn run_str(&self, text: &'a str) -> Result<T, ParseFailure<'a>> {
        let chars: Vec<char> = text.chars().collect();
        self.clone().run(&chars)
    }
}

#[test]
fn test_satisfy() {
    let s1: Vec<char> = "abc".chars().collect();
    let s2: Vec<char> = "bcd".chars().collect();
    let p = satisfy(|&ch| ch == 'a');

    assert_eq!(p.clone().run(&s1).expect("should be Ok"), 'a');
    assert!(p.clone().run(&s2).is_err())
}
