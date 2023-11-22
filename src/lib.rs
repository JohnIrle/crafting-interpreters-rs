mod scanner;
mod token_type;
mod token;
mod runner;
mod expr;
mod ast_printer;

pub use runner::Runner;
pub use token_type::TokenType;
pub use token::Token;
pub use scanner::Scanner;
pub use expr::Expr;

pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: usize, location: String, message: String) {
    println!("[line {}] Error {}: {}", line, location, message);
}