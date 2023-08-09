use std::fmt::Display;

use crate::lexer::Token;

pub trait Expression: Display {
    fn eval(&self) -> f64;
}

pub struct AddOp(pub Box<dyn Expression>, pub Box<dyn Expression>);
pub struct SubOp(pub Box<dyn Expression>, pub Box<dyn Expression>);
pub struct MulOp(pub Box<dyn Expression>, pub Box<dyn Expression>);
pub struct DivOp(pub Box<dyn Expression>, pub Box<dyn Expression>);
pub struct Number(pub f64);

impl Expression for AddOp {
    fn eval(&self) -> f64 {
        self.0.eval() + self.1.eval()
    }
}

impl Display for AddOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.0, self.1)
    }
}

impl Expression for SubOp {
    fn eval(&self) -> f64 {
        self.0.eval() - self.1.eval()
    }
}

impl Display for SubOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} - {})", self.0, self.1)
    }
}

impl Expression for MulOp {
    fn eval(&self) -> f64 {
        self.0.eval() * self.1.eval()
    }
}

impl Display for MulOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.0, self.1)
    }
}

impl Expression for DivOp {
    fn eval(&self) -> f64 {
        self.0.eval() / self.1.eval()
    }
}

impl Display for DivOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.0, self.1)
    }
}

impl Expression for Number {
    fn eval(&self) -> f64 {
        self.0
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<Token> for Number {
    type Error = (&'static str, Token);
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Token::Number(val) = value {
            Ok(Number(val))
        } else {
            Err((&"invalid token type", value))
        }
    }
}
