// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT
//
// This file is part of rlox-ast

mod ast_printer;
mod expr;
mod object;
mod parser;
mod runner;
mod scanner;
mod token;
mod token_type;

pub use expr::Expr;
pub use parser::Parser;
pub use runner::Runner;
pub use scanner::Scanner;
pub use token::Token;
pub use token_type::TokenType;

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
