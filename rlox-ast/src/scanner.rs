// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT
//
// This file is part of rlox-ast

use crate::object::Object;
use crate::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        if let Some(character) = self.source.chars().nth(self.current - 1) {
            character
        } else {
            panic!("No character found at index {}", self.current - 1);
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    crate::line_error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = Self::keywords(text);
        let token_type = match token_type {
            Some(token_type) => token_type.clone(),
            None => TokenType::IDENTIFIER,
        };
        self.add_token(token_type);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
        }

        while self.is_digit(self.peek()) {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        let num = value.parse().unwrap();
        self.add_token_literal(TokenType::NUMBER, Some(Object::Num(num)));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::line_error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::STRING, Some(Object::Str(value.to_string())));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(character) = self.source.chars().nth(self.current) {
            if character != expected {
                return false;
            }
        } else {
            panic!("No character found at index {}", self.current);
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        if let Some(character) = self.source.chars().nth(self.current) {
            character
        } else {
            panic!("No character found at index {}", self.current);
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        if let Some(character) = self.source.chars().nth(self.current + 1) {
            character
        } else {
            panic!("No character found at index {}", self.current + 1);
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn keywords(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "else" => Some(TokenType::ELSE),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_lexemes() {
        let mut scanner = Scanner::new("(){},.-+*;".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::LEFT_PAREN, "(".to_string(), None, 0),
                Token::new(TokenType::RIGHT_PAREN, ")".to_string(), None, 0),
                Token::new(TokenType::LEFT_BRACE, "{".to_string(), None, 0),
                Token::new(TokenType::RIGHT_BRACE, "}".to_string(), None, 0),
                Token::new(TokenType::COMMA, ",".to_string(), None, 0),
                Token::new(TokenType::DOT, ".".to_string(), None, 0),
                Token::new(TokenType::MINUS, "-".to_string(), None, 0),
                Token::new(TokenType::PLUS, "+".to_string(), None, 0),
                Token::new(TokenType::STAR, "*".to_string(), None, 0),
                Token::new(TokenType::SEMICOLON, ";".to_string(), None, 0),
                Token::new(TokenType::EOF, "".to_string(), None, 0),
            ]
        );
    }

    #[test]
    fn test_operators() {
        let mut scanner = Scanner::new("! != = == < <= > >=".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::BANG, "!".to_string(), None, 0),
                Token::new(TokenType::BANG_EQUAL, "!=".to_string(), None, 0),
                Token::new(TokenType::EQUAL, "=".to_string(), None, 0),
                Token::new(TokenType::EQUAL_EQUAL, "==".to_string(), None, 0),
                Token::new(TokenType::LESS, "<".to_string(), None, 0),
                Token::new(TokenType::LESS_EQUAL, "<=".to_string(), None, 0),
                Token::new(TokenType::GREATER, ">".to_string(), None, 0),
                Token::new(TokenType::GREATER_EQUAL, ">=".to_string(), None, 0),
                Token::new(TokenType::EOF, "".to_string(), None, 0),
            ]
        );
    }

    #[test]
    fn test_comments() {
        let mut scanner = Scanner::new("// stufff herer comments".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![Token::new(TokenType::EOF, "".to_string(), None, 0),]
        );
    }

    #[test]
    fn test_slash() {
        let mut scanner = Scanner::new("/".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::SLASH, "/".to_string(), None, 0),
                Token::new(TokenType::EOF, "".to_string(), None, 0),
            ]
        );
    }

    #[test]
    fn test_line_increment() {
        let mut scanner = Scanner::new("/\n*\n".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::SLASH, "/".to_string(), None, 0),
                Token::new(TokenType::STAR, "*".to_string(), None, 1),
                Token::new(TokenType::EOF, "".to_string(), None, 2),
            ]
        );
    }

    #[test]
    fn test_string_literal() {
        let mut scanner = Scanner::new("\"this is a string\"".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenType::STRING,
                    "\"this is a string\"".to_string(),
                    Some(Object::Str("this is a string".to_string())),
                    0
                ),
                Token::new(TokenType::EOF, "".to_string(), None, 0),
            ]
        );
    }

    #[test]
    fn test_whole_number() {
        let mut scanner = Scanner::new("123".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenType::NUMBER,
                    "123".to_string(),
                    Some(Object::Num(123_f64)),
                    0
                ),
                Token::new(TokenType::EOF, "".to_string(), None, 0),
            ]
        );
    }

    #[test]
    fn test_decimal_number() {
        let mut scanner = Scanner::new("123.456".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenType::NUMBER,
                    "123.456".to_string(),
                    Some(Object::Num(123.456)),
                    0
                ),
                Token::new(TokenType::EOF, "".to_string(), None, 0),
            ]
        );
    }

    #[test]
    fn test_keywords() {
        let mut scanner = Scanner::new("or and class".to_string());

        let tokens = scanner.scan_tokens();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::OR, "or".to_string(), None, 0),
                Token::new(TokenType::AND, "and".to_string(), None, 0),
                Token::new(TokenType::CLASS, "class".to_string(), None, 0),
                Token::new(TokenType::EOF, "".to_string(), None, 0),
            ]
        );
    }
}
