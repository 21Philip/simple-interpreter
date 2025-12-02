use lib_parse::combinators::*;
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

#[test]
fn test_pchar() {
    let p = pchar('a');

    assert_eq!(p.run_str("abc").expect("should be Ok"), 'a');
    assert!(p.run_str("bcd").is_err());
}

#[test]
fn test_pstring() {
    let p = pstring("test");

    assert_eq!(p.run_str("testabab").expect("should be Ok"), "test");
    assert!(p.run_str("tesabab").is_err());
}

#[test]
fn test_choice() {
    let p = choice(&[pchar('a'), pchar('b'), pchar('c')]);

    assert_eq!(p.run_str("bca").expect("should be Ok"), 'b');
    assert!(p.run_str("def").is_err());
}

#[test]
fn test_sequence() {
    let p = sequence(&[pchar('a'), pchar('b'), pchar('c')]);

    assert_eq!(p.run_str("abcd").expect("should be Ok"), ['a', 'b', 'c']);
    assert!(p.run_str("abdc").is_err());
}

#[test]
fn test_many() {
    let p = many(&pchar('a'));

    assert_eq!(p.run_str("aaabb").expect("should be Ok"), ['a', 'a', 'a']);
    assert_eq!(p.run_str("abb").expect("should be Ok"), ['a']);
    assert_eq!(p.run_str("bb").expect("should be Ok"), []);
}

#[test]
fn test_many1() {
    let p = many1(&pchar('a'));

    assert_eq!(p.run_str("aaabb").expect("should be Ok"), ['a', 'a', 'a']);
    assert_eq!(p.run_str("abb").expect("should be Ok"), ['a']);
    assert!(p.run_str("bb").is_err());
}

#[test]
fn test_then() {
    let p = pchar('a').then(&pstring("b"));

    assert_eq!(p.run_str("ab").expect("should be Ok"), "b");
    assert!(p.run_str("ac").is_err());
    assert!(p.run_str("cb").is_err());
}

#[test]
fn test_before() {
    let p = pchar('a').before(&pstring("b"));

    assert_eq!(p.run_str("ab").expect("should be Ok"), 'a');
    assert!(p.run_str("ac").is_err());
    assert!(p.run_str("cb").is_err());
}

#[test]
fn test_and() {
    let p = pchar('a').and(&pstring("b"));

    assert_eq!(
        p.run_str("ab").expect("should be Ok"),
        ('a', "b".to_string())
    );
    assert!(p.run_str("ac").is_err());
    assert!(p.run_str("cb").is_err());
}

#[test]
fn test_map() {
    let p = sequence(&[pchar('a'), pchar('b'), pchar('c')]).map(String::from_iter);

    assert_eq!(p.run_str("abcd").expect("should be Ok"), "abc");
    assert!(p.run_str("abdc").is_err());
}
