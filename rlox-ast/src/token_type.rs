// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT
//
// This file is part of rlox-ast

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LEFT_PAREN => write!(f, "LEFT_PAREN"),
            Self::RIGHT_PAREN => write!(f, "RIGHT_PAREN"),
            Self::LEFT_BRACE => write!(f, "LEFT_BRACE"),
            Self::RIGHT_BRACE => write!(f, "RIGHT_BRACE"),
            Self::COMMA => write!(f, "COMMA"),
            Self::DOT => write!(f, "DOT"),
            Self::MINUS => write!(f, "MINUS"),
            Self::PLUS => write!(f, "PLUS"),
            Self::SEMICOLON => write!(f, "SEMICOLON"),
            Self::SLASH => write!(f, "SLASH"),
            Self::STAR => write!(f, "STAR"),
            Self::BANG => write!(f, "BANG"),
            Self::BANG_EQUAL => write!(f, "BANG_EQUAL"),
            Self::EQUAL => write!(f, "EQUAL"),
            Self::EQUAL_EQUAL => write!(f, "EQUAL_EQUAL"),
            Self::GREATER => write!(f, "GREATER"),
            Self::GREATER_EQUAL => write!(f, "GREATER_EQUAL"),
            Self::LESS => write!(f, "LESS"),
            Self::LESS_EQUAL => write!(f, "LESS_EQUAL"),
            Self::IDENTIFIER => write!(f, "IDENTIFIER"),
            Self::STRING => write!(f, "STRING"),
            Self::NUMBER => write!(f, "NUMBER"),
            Self::AND => write!(f, "AND"),
            Self::CLASS => write!(f, "CLASS"),
            Self::ELSE => write!(f, "ELSE"),
            Self::FALSE => write!(f, "FALSE"),
            Self::FUN => write!(f, "FUN"),
            Self::FOR => write!(f, "FOR"),
            Self::IF => write!(f, "IF"),
            Self::NIL => write!(f, "NIL"),
            Self::OR => write!(f, "OR"),
            Self::PRINT => write!(f, "PRINT"),
            Self::RETURN => write!(f, "RETURN"),
            Self::SUPER => write!(f, "SUPER"),
            Self::THIS => write!(f, "THIS"),
            Self::TRUE => write!(f, "TRUE"),
            Self::VAR => write!(f, "VAR"),
            Self::WHILE => write!(f, "WHILE"),
            Self::EOF => write!(f, "EOF"),
        }
    }
}
