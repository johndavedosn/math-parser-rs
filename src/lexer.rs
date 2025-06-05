use logos::Logos;
use logos::Lexer;
#[macro_export]
macro_rules! string {
    ($str:literal) => {{
        $str.to_string()
    }};
}
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Tokens {
    #[regex(r"\d+", |lex| lex.slice().parse::<i64>().unwrap())]
    Number(i64),
    #[regex(r"[A-Za-z]+",  |lex| lex.slice().to_string())]
    Variable(String),
    #[regex(r"[+\-*/^=]", which_operation)]
    Operation(Option<Operations>),
    #[regex(r"[A-Za-z]+\([A-Za-z0-9+\-*/^= ]*\)", |lex| lex.slice().to_string())]
    Function(String)
}
#[derive(Debug, PartialEq)]
pub enum Operations {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponantiation,
    Equals
}
fn which_operation<'s>(lex: &mut Lexer<'s, Tokens>) -> Option<Operations>{
    match lex.slice() {
        "+" => Some(Operations::Addition),
        "-" => Some(Operations::Subtraction),
        "*" => Some(Operations::Multiplication),
        "/" => Some(Operations::Division),
        "^" => Some(Operations::Exponantiation),
        "=" => Some(Operations::Equals),
        _ => None
    }
}
pub mod tests {
    #[allow(unused_imports)]
    use logos::Logos;
    #[allow(unused_imports)]
    use crate::lexer::Operations;

    #[cfg(test)]
    use super::Tokens;
    #[test] 
    pub fn test_lexer_1() {
        let mut lex = Tokens::lexer("1 + 1 = 3x");
        assert_eq!(lex.next(),  Some(Ok(Tokens::Number(1))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Operation(Some(Operations::Addition)))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(1))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Operation(Some(Operations::Equals)))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(3))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Variable(string!("x")))));
    }
    #[test]
    pub fn test_lexer_2() {
        let mut lex = Tokens::lexer("f(x) = 5x^2 + 3 - 4");
        assert_eq!(lex.next(), Some(Ok(Tokens::Function(string!("f(x)")))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Operation(Some(Operations::Equals)))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(5))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Variable(string!("x")))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Operation(Some(Operations::Exponantiation)))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(2))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Operation(Some(Operations::Addition)))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(3))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Operation(Some(Operations::Subtraction)))));
        assert_eq!(lex.next(), Some(Ok(Tokens::Number(4))));

    }
}
