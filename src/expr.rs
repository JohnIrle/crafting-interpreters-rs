use crate::Token;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(String),
    Unary(Token, Box<Expr>),
}

pub trait Visitor {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String;
    fn visit_grouping_expr(&self, expression: &Expr) -> String;
    fn visit_literal_expr(&self, value: &str) -> String;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String;
}

pub trait Accept {
    fn accept<V: Visitor>(&self, visitor: &V) -> String;
}

impl Accept for Expr {
    fn accept<V: Visitor>(&self, visitor: &V) -> String {
        match self {
            Expr::Binary(left, operator, right) => {
                visitor.visit_binary_expr(left, operator, right)
            },
            Expr::Grouping(expression) => {
                visitor.visit_grouping_expr(expression)
            },
            Expr::Literal(value) => {
                visitor.visit_literal_expr(value)
            },
            Expr::Unary(operator, right) => {
                visitor.visit_unary_expr(operator, right)
            },
        }
    }
}
