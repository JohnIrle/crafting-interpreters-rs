mod scanner;
mod token_type;
mod token;
mod runner;

pub use runner::Runner;
pub use token_type::TokenType;
pub use token::Token;
pub use scanner::Scanner;

pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: usize, location: String, message: String) {
    println!("[line {}] Error {}: {}", line, location, message);
}