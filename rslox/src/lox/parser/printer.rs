#[cfg(test)]
mod test;

use std::fmt::Write;

use crate::lox::parser::{
    expressions::{Binary, Expr, Grouping, Literal, LiteralValue, Unary},
    visitor::Visitor,
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expr: &Expr) -> String {
        expr.accept(&mut AstPrinter {})
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary: &Binary) -> String {
        parenthesize(&binary.operator.lexeme, &[&binary.left, &binary.right])
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> String {
        parenthesize("group", &[&grouping.expression])
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        match &literal.value {
            LiteralValue::String(str) => str.to_owned(),
            LiteralValue::Number(number) => number.to_string(),
            LiteralValue::Boolean(boolean) => boolean.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    fn visit_unary(&mut self, unary: &Unary) -> String {
        parenthesize(&unary.operator.lexeme, &[&unary.right])
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut str = String::new();
    write!(str, "({name}").unwrap();

    for expr in exprs {
        write!(str, " {}", AstPrinter::print(expr)).unwrap();
    }

    write!(str, ")").unwrap();
    str
}
