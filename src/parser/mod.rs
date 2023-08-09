use std::vec::IntoIter;

use crate::lexer::Token;

use self::nodes::{AddOp, DivOp, Expression, MulOp, Number, SubOp};

mod nodes;

pub struct Parser {
    tokens: IntoIter<Token>,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Parser {
            tokens: token_stream.into_iter(),
            current_token: None,
        }
    }

    pub fn parse(mut self) -> Box<dyn Expression> {
        self.next_token();
        let result = self.parse_E();
        self.expect(&None);
        result
    }

    fn next_token(&mut self) {
        self.current_token = self.tokens.next();
    }

    fn expect(&mut self, token: &Option<Token>) {
        let current_token = &self.current_token;
        if current_token != token {
            panic!("invalid token {current_token:?} expected {token:?}")
        }
        self.next_token();
    }

    #[allow(non_snake_case)]
    fn parse_E(&mut self) -> Box<dyn Expression> {
        let expr = self.parse_T();
        self.parse_Ep(expr)
    }

    #[allow(non_snake_case)]
    fn parse_Ep(&mut self, expr: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = &self.current_token;

        match token {
            Some(Token::Plus) => {
                self.next_token();
                let rhs = self.parse_T();
                let expr = Box::new(AddOp(expr, rhs));
                self.parse_Ep(expr)
            }
            Some(Token::Minus) => {
                self.next_token();
                let rhs = self.parse_T();
                let expr = Box::new(SubOp(expr, rhs));
                self.parse_Ep(expr)
            }
            None | Some(Token::RParen) => expr,
            _ => panic!("invalid token {token:?} expected +, -, ), #"),
        }
    }

    #[allow(non_snake_case)]
    fn parse_T(&mut self) -> Box<dyn Expression> {
        let expr = self.parse_F();
        self.parse_Tp(expr)
    }

    #[allow(non_snake_case)]
    fn parse_Tp(&mut self, expr: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = &self.current_token;

        match token {
            Some(Token::Star) => {
                self.next_token();
                let rhs = self.parse_F();
                let expr = Box::new(MulOp(expr, rhs));
                self.parse_Tp(expr)
            }
            Some(Token::Slash) => {
                self.next_token();
                let rhs = self.parse_F();
                let expr = Box::new(DivOp(expr, rhs));
                self.parse_Tp(expr)
            }
            None | Some(Token::Plus | Token::Minus | Token::RParen) => expr,
            _ => panic!("invalid token {token:?} expected *, /, +, -, #"),
        }
    }

    #[allow(non_snake_case)]
    fn parse_F(&mut self) -> Box<dyn Expression> {
        let token = &self.current_token;

        match token {
            Some(Token::LParen) => {
                self.next_token();
                let expr = self.parse_E();
                self.expect(&Some(Token::RParen));
                expr
            }
            Some(Token::Number(value)) => {
                let expr = Box::new(Number(*value));
                self.next_token();
                expr
            }
            _ => panic!("invalid token {token:?} expected (, number"),
        }
    }
}
