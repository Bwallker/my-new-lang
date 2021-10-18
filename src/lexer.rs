pub enum TokenType {
    PLUS,
    MINUS,
    DIVIDE,
    MULTIPLY
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self);
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}, {}", self.token_type, self.value);
    }
}