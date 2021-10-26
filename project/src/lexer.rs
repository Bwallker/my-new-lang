mod src {
    mod lib {
        pub mod lib;
    }
}
use lib::token_str;

#[define(token_str)]
pub enum Token {
    Plus,
    Minus,
    Divide,
    Multiply,
    Colon,
    Semicolon,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    Int(i64),
    Float(f64),
    String(String),
}
impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_str: &'static str = self.into();
        return write!(f, "{}", as_str);
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_str: &'static str = self.into();
        return write!(f, "{}", as_str);
    }
}
fn lexer(input: String) -> Vec<Token> {
    return Vec::new();
}
