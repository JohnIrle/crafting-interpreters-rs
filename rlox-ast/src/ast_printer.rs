use crate::expr::{Accept, Binary, Expr, Grouping, Literal, Unary, Visitor};

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
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        self.parenthesize(expr.operator.lexeme(), vec![&*expr.left, &*expr.right])
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        self.parenthesize("group", vec![&*expr.expression])
    }

    fn visit_literal_expr(&self, expr: &Literal) -> String {
        if expr.value.is_empty() {
            return "nil".to_string();
        }
        expr.value.to_string()
    }

    fn visit_unary_expr(&self, expr: &Unary) -> String {
        self.parenthesize(expr.operator.lexeme(), vec![&*expr.right])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;
    use crate::TokenType;

    #[test]
    fn test_ast_print() {
        let ast_printer = AstPrinter::new();
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: Token::new(TokenType::MINUS, "-".to_string(), None, 1),
                right: Box::new(Expr::Literal(Literal {
                    value: "123".to_string(),
                })),
            })),
            operator: Token::new(TokenType::STAR, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping(Grouping {
                expression: Box::new(Expr::Literal(Literal {
                    value: "45.67".to_string(),
                })),
            })),
        });

        assert_eq!(ast_printer.print(&expr), "(* (- 123) (group 45.67))");
    }
}
