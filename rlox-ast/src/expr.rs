use crate::Token;

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}
pub struct Grouping {
    pub expression: Box<Expr>
}
pub struct Literal {
    pub value: String
}
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>
}
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Binary) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_unary_expr(&self, expr: &Unary) -> T;
}

pub trait Accept<T> {
    fn accept<V: Visitor<T>>(&self, visitor: &V) -> T;
}

impl Accept<String> for Expr {
    fn accept<V: Visitor<String>>(&self, visitor: &V) -> String {
        match self {
            Expr::Binary(expr) => visitor.visit_binary_expr(expr),
            Expr::Grouping(expr) => visitor.visit_grouping_expr(expr),
            Expr::Literal(expr) => visitor.visit_literal_expr(expr),
            Expr::Unary(expr) => visitor.visit_unary_expr(expr),
        }
    }
}
