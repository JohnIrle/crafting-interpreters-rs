mod scanner;
mod token_type;
mod token;
mod runner;
mod expr;
mod ast_printer;
mod object;
mod parser;

pub use runner::Runner;
pub use token_type::TokenType;
pub use token::Token;
pub use scanner::Scanner;
pub use expr::Expr;
pub use parser::Parser;

pub fn line_error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, location, message);
}


fn error(token: &Token, message: &str) {
    if *token.token_type() == TokenType::EOF {
        report(token.line(), " at end", message);
    } else {
        report(token.line(), &format!(" at '{}'", token.lexeme()), message);
    }
}
