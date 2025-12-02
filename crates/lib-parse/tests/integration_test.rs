use lib_parse::prelude::*;

trait ParserTestExt<T> {
    fn run_str(&self, s: &str) -> Result<T, ParseFailure>;
}

impl<T: 'static> ParserTestExt<T> for Parser<T> {
    fn run_str(&self, s: &str) -> Result<T, ParseFailure> {
        let chars: Vec<char> = s.chars().collect();
        self.clone().run(&chars)
    }
}

#[test]
fn test_satisfy() {
    let p = satisfy(|&ch| ch == 'a');

    assert_eq!(p.run_str("abc").expect("should be Ok"), 'a');
    assert!(p.run_str("bcd").is_err())
}
