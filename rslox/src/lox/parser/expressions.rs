use crate::lox::{token::Token, types};

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Expr {
        Expr::Binary(Binary {
            left,
            operator,
            right,
        })
    }
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expr: Box<Expr>) -> Expr {
        Expr::Grouping(Grouping { expression: expr })
    }
}

pub enum LiteralValue {
    String(String),
    Number(types::Number),
    Boolean(bool),
    Nil,
}

pub struct Literal {
    pub value: LiteralValue,
}

impl Literal {
    pub fn new(value: LiteralValue) -> Expr {
        Expr::Literal(Literal { value })
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Expr {
        Expr::Unary(Unary { operator, right })
    }
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}
