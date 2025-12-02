use lib_parse::prelude::*;

trait ParserTestExt<T> {
    fn run_str(self, s: &str) -> Result<T, ParseFailure>;
}

impl<'a, T> ParserTestExt<T> for Parser<'a, T> {
    fn run_str(self, text: &str) -> Result<T, ParseFailure> {
        let chars: Vec<char> = text.chars().collect();
        let result = self.clone().run(&chars);
        result
    }
}

#[test]
fn test_satisfy() {
    //let s1: Vec<char> = "abc".chars().collect();
    //let s2: Vec<char> = "bcd".chars().collect();

    let p = satisfy(|&ch| ch == 'a');
    let result = p.run_str("hej");

    //assert_eq!(p.clone().run(&s1).expect("should be Ok"), 'a');
    //assert!(p.clone().run(&s2).is_err())
}
