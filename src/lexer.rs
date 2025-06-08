use logos::Logos;
#[macro_export]
macro_rules! string {
    ($str:literal) => {{
        $str.to_string()
    }}
}
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Tokens {
    #[regex(r"\d+", |lex| lex.slice().parse::<i64>().unwrap())]
    Number(i64),
    #[regex(r"[A-Za-z]+",  |lex| lex.slice().to_string())]
    Variable(String),
    #[token("+")]
    Addition,
    #[token("-")]
    Subtraction,
    #[token("*")]
    Multiplication,
    #[token("/")]
    Division,
    #[token("^")]
    Exponantiation,
    #[token("=")]
    Equals,
    #[token("(")]
    OpenParan,
    #[token(")")]
    CloseParan
}
pub fn lex(input: &str) -> Vec<Tokens>{
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut lexer = Tokens::lexer(input);
    while let Some(token) = lexer.next() {
        match token {
            Ok(tk) => {
                tokens.push(tk);
            },
            Err(_) => {
                // I still need to figure out what to do here.
            }
        }
    }
    tokens
}
#[cfg(test)]
pub mod tests {
    #[allow(unused_imports)]
    use logos::Logos;
    #[allow(unused_imports)]
    use crate::lexer::lex;

    use super::Tokens;
    #[test] 
    pub fn test_lexer_1() {
        let mut lex = Tokens::lexer("1 + 1 = 3x");
        assert_eq!(lex.next(),  Some(Ok(Tokens::Number(1))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Addition)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(1))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Equals)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(3))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Variable(string!("x")))));
    }
    #[test]
    pub fn test_lexer_2() {
        let mut lex = Tokens::lexer("f(x) = 5x^2 + 3 - 4");
        assert_eq!(lex.next(), Some(Ok(Tokens::Variable(string!("f")))));
        assert_eq!(lex.next(), Some(Ok(Tokens::OpenParan)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Variable(string!("x")))));
        assert_eq!(lex.next(), Some(Ok(Tokens::CloseParan)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Equals)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(5))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Variable(string!("x")))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Exponantiation)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(2))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Addition)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(3))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Subtraction)));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(4))));

    }
    #[test]
    pub fn test_lexer_3() {
        let input = "f(x) = 5x^2 + 3 - 4";
        let expected_tokens = vec![Tokens::Variable(string!("f")), Tokens::OpenParan, Tokens::Variable(string!("x")), Tokens::CloseParan, Tokens::Equals, Tokens::Number(5), Tokens::Variable(string!("x")), Tokens::Exponantiation, Tokens::Number(2), Tokens::Addition, Tokens::Number(3), Tokens::Subtraction, Tokens::Number(4)];
        assert_eq!(lex(input), expected_tokens);
 
    }
}