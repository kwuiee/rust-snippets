use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,
    #[token(".")]
    Period,
    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,
    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn main() {
    let mut lex = Token::lexer("Create ridiculously fast Lexers.");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.span(), 0..6);
    assert_eq!(lex.slice(), "Create");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.span(), 7..19);
    assert_eq!(lex.slice(), "ridiculously");

    assert_eq!(lex.next(), Some(Token::Fast));
    assert_eq!(lex.span(), 20..24);
    assert_eq!(lex.slice(), "fast");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.slice(), "Lexers");
    assert_eq!(lex.span(), 25..31);

    assert_eq!(lex.next(), Some(Token::Period));
    assert_eq!(lex.span(), 31..32);
    assert_eq!(lex.slice(), ".");

    assert_eq!(lex.next(), None);

    // Callback
    let mut lex = Callback::lexer("5 42k 75m");

    assert_eq!(lex.next(), Some(Callback::Number(5)));
    assert_eq!(lex.slice(), "5");

    assert_eq!(lex.next(), Some(Callback::Number(42_000)));
    assert_eq!(lex.slice(), "42k");

    assert_eq!(lex.next(), Some(Callback::Number(75_000_000)));
    assert_eq!(lex.slice(), "75m");

    assert_eq!(lex.next(), None);
}

// Note: callbacks can return `Option` or `Result`
fn kilo(lex: &mut Lexer<Callback>) -> Option<u64> {
    let slice = lex.slice();
    let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
    Some(n * 1_000)
}

fn mega(lex: &mut Lexer<Callback>) -> Option<u64> {
    let slice = lex.slice();
    let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
    Some(n * 1_000_000)
}

#[derive(Logos, Debug, PartialEq)]
enum Callback {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
    // Callbacks can use closure syntax, or refer
    // to a function defined elsewhere.
    //
    // Each pattern can have it's own callback.
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    #[regex("[0-9]+k", kilo)]
    #[regex("[0-9]+m", mega)]
    Number(u64),
}
