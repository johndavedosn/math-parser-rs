#![allow(unused)]
use crate::lexer::Tokens;
#[derive(PartialEq)]
pub enum Assoc {
    Left,
    Right
}
fn precedence(token: &Tokens) -> Option<(u8, Assoc)> {
    match token {
        Tokens::Addition | Tokens::Subtraction => Some((2, Assoc::Left)),
        Tokens::Multiplication | Tokens::Division => Some((3, Assoc::Left)),
        Tokens::Exponantiation => Some((4, Assoc::Right)),
        _ => None,
    } 
}
pub fn shunting_yard(tokens: Vec<Tokens>) -> Vec<Tokens> {
    let mut output: Vec<Tokens> = Vec::new();
    let mut ops: Vec<Tokens> = Vec::new();

    for token in tokens {
        match token {
            Tokens::Number(_) | Tokens::Variable(_) => {
                output.push(token);
            }
            Tokens::Addition
            | Tokens::Subtraction
            | Tokens::Multiplication
            | Tokens::Division
            | Tokens::Exponantiation => {
                while let Some(top) = ops.last() {
                    if let Some((prec1, assoc1)) = precedence(&token) {
                        if let Some((prec2, _)) = precedence(top) {
                            if (assoc1 == Assoc::Left && prec1 <= prec2)
                                || (assoc1 == Assoc::Right && prec1 < prec2)
                            {
                                output.push(ops.pop().unwrap());
                                continue;
                            }
                        }
                    }
                    break;
                }
                ops.push(token);
            }
            Tokens::OpenParan => {
                ops.push(token);
            }
            Tokens::CloseParan => {
                while let Some(top) = ops.last() {
                    if *top == Tokens::OpenParan {
                        ops.pop();
                        break;
                    } else {
                        output.push(ops.pop().unwrap());
                    }
                }
            }
            _ => {
                // skip Equals or handle as assignment separately later
            }
        }
    }

    while let Some(op) = ops.pop() {
        output.push(op);
    }


    output
}
#[cfg(test)]
pub mod tests {
    #![allow(unused)]
    use crate::parser::shunting_yard;
    use crate::lexer::Tokens;
    #[test]
    pub fn test_parser_1(){
        let expr = vec![
            Tokens::Number(3),
            Tokens::Addition,
            Tokens::Number(4),
            Tokens::Multiplication,
            Tokens::Number(2),
            Tokens::Exponantiation,
            Tokens::OpenParan,
            Tokens::Number(1),
            Tokens::Subtraction,
            Tokens::Number(5),
            Tokens::CloseParan
        ];
        let expected_out = vec![
            Tokens::Number(3),
            Tokens::Number(4),
            Tokens::Number(2),
           Tokens::Number(1),
            Tokens::Number(5),
           Tokens::Subtraction,
           Tokens::Exponantiation,
           Tokens::Multiplication,
            Tokens::Addition
        ];
        assert_eq!(expected_out, shunting_yard(expr));
    }
}