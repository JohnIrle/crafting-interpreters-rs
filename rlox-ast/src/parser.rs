// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT

use TokenType::{
    BANG_EQUAL, EQUAL_EQUAL, FALSE, GREATER, GREATER_EQUAL, LEFT_PAREN, LESS, LESS_EQUAL, MINUS,
    NIL, NUMBER, PLUS, RIGHT_PAREN, STRING, TRUE,
};

use crate::TokenType::{BANG, SLASH, STAR};
use crate::{error, Expr, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison();

        while self.match_types(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                operator,
                Box::new(right.unwrap()),
            ));
        }

        expr
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term();

        while self.match_types(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();
            let right = self.term();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                operator,
                Box::new(right.unwrap()),
            ));
        }

        expr
    }

    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor();

        while self.match_types(vec![MINUS, PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                operator,
                Box::new(right.unwrap()),
            ));
        }

        expr
    }

    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary();

        while self.match_types(vec![SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Some(Expr::Binary(
                Box::new(expr.unwrap()),
                operator,
                Box::new(right.unwrap()),
            ));
        }

        expr
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_types(vec![BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Some(Expr::Unary(operator, Box::new(right.unwrap())));
        }
        self.primary()
    }

    // TODO: Return Result?
    fn primary(&mut self) -> Option<Expr> {
        if self.match_types(vec![FALSE]) {
            return Some(Expr::Literal("false".to_string()));
        }
        if self.match_types(vec![TRUE]) {
            return Some(Expr::Literal("true".to_string()));
        }
        if self.match_types(vec![NIL]) {
            return Some(Expr::Literal("nil".to_string()));
        }
        if self.match_types(vec![NUMBER, STRING]) {
            // TODO: This is a bit of a hack, but it works for now.
            return Some(Expr::Literal(
                self.previous()
                    .literal()
                    .expect("could not unwrap")
                    .to_string(),
            ));
        }
        if self.match_types(vec![LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "Expect ')' after expression.");
            return Some(Expr::Grouping(Box::new(expr.unwrap())));
        }
        self.error("Expect expression.");
        None
    }

    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Option<Token> {
        if self.check(token_type) {
            Some(self.advance())
        } else {
            self.error(message);
            None
        }
    }

    fn error(&mut self, message: &str) {
        let token = self.peek();
        error(token, message);
        panic!();
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == &TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type() {
                TokenType::CLASS => return,
                TokenType::FUN => return,
                TokenType::VAR => return,
                TokenType::FOR => return,
                TokenType::IF => return,
                TokenType::WHILE => return,
                TokenType::PRINT => return,
                TokenType::RETURN => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        *self.peek().token_type() == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        *self.peek().token_type() == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
