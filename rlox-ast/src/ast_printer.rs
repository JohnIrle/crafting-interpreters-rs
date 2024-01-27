use crate::{Token};
use crate::expr::{Accept, Visitor, Expr};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut builder = format!("({}", name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}


impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme(), vec![left, right])
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthesize("group", vec![expression])
    }

    fn visit_literal_expr(&self, value: &str) -> String {
        if value.is_empty() {
            return "nil".to_string();
        }
        value.to_string()
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme(), vec![right])
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenType;
    use super::*;

    #[test]
    fn test_ast_print() {
        let ast_printer = AstPrinter::new();
        let expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::MINUS, "-".to_string(), None, 1),
                Box::new(Expr::Literal("123".to_string()))
            )),
            Token::new(TokenType::STAR, "*".to_string(), None, 1),
            Box::new(Expr::Grouping(
                Box::new(Expr::Literal("45.67".to_string())))));

        assert_eq!(ast_printer.print(&expr), "(* (- 123) (group 45.67))");
    }
}