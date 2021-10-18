mod lexer;
fn main() {

    let token = lexer::Token{token_type: lexer::TokenType::MINUS, value: String::from("")};
    println!("{}", token);
}
