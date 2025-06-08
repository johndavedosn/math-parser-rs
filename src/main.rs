mod lexer;
mod parser;
fn main() {
    println!("{:?}", lexer::lex("f(x) = 5x^2 + 3 - 4"));
}
