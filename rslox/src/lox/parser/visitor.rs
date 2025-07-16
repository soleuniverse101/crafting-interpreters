use crate::lox::parser::expressions::{Binary, Expr, Grouping, Literal, LiteralValue, Unary};

pub trait Visitor<T> {
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Unary(unary) => visitor.visit_unary(unary),
        }
    }
}
