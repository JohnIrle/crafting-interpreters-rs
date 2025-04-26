// SPDX-FileCopyrightText: 2024 John Irle
// SPDX-License-Identifier: MIT
//
// This file is part of rlox-ast

use crate::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(String),
    Unary(Token, Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&self, expression: &Expr) -> T;
    fn visit_literal_expr(&self, value: &str) -> T;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> T;
}

pub trait Accept<T> {
    fn accept<V: Visitor<T>>(&self, visitor: &V) -> T;
}

impl Accept<String> for Expr {
    fn accept<V: Visitor<String>>(&self, visitor: &V) -> String {
        match self {
            Self::Binary(left, operator, right) => visitor.visit_binary_expr(left, operator, right),
            Self::Grouping(expression) => visitor.visit_grouping_expr(expression),
            Self::Literal(value) => visitor.visit_literal_expr(value),
            Self::Unary(operator, right) => visitor.visit_unary_expr(operator, right),
        }
    }
}
