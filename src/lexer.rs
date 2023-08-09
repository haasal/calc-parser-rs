use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"\d*\.?\d+(?:[eE][-+]?\d+)?", |lex| lex.slice().parse().ok())]
    Number(f64),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
}

pub fn get_token_stream(source: &str) -> Vec<Token> {
    Token::lexer(source)
        .map(|t| t.expect("lexing error on {t:?}"))
        .collect()
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::{Token, Token::*};

    #[test]
    fn test_valid_code() {
        let code = "(23-43.1)/7.23e-13*12";
        let mut lexer = Token::lexer(code);
        assert_eq!(lexer.next(), Some(Ok(LParen)));
        assert_eq!(lexer.next(), Some(Ok(Number(23.0))));
        assert_eq!(lexer.next(), Some(Ok(Minus)));
        assert_eq!(lexer.next(), Some(Ok(Number(43.1))));
        assert_eq!(lexer.next(), Some(Ok(RParen)));
        assert_eq!(lexer.next(), Some(Ok(Slash)));
        assert_eq!(lexer.next(), Some(Ok(Number(7.23e-13))));
        assert_eq!(lexer.next(), Some(Ok(Star)));
        assert_eq!(lexer.next(), Some(Ok(Number(12.0))));
    }
}
