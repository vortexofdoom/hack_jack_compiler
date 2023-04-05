use std::fmt::{Debug, Display};

use crate::tokens::{
    Keyword::{self, *},
    Token,
};

pub trait ValidToken: Display + Debug + PartialEq<TokenType> {}

impl PartialEq<TokenType> for Box<dyn ValidToken> {
    fn eq(&self, other: &TokenType) -> bool {
        self.as_ref() == other
    }
}
impl ValidToken for Token {}
impl ValidToken for Keyword {}
impl ValidToken for char {}
impl ValidToken for i16 {}
impl ValidToken for String {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    ClassVarDec,
    Constant,
    Name,
    BinaryOp,
    UnaryOp,
    Statement,
    SubroutineDec,
    Type,
    ReturnType,
}
impl ValidToken for TokenType {}
impl PartialEq<Token> for TokenType {
    fn eq(&self, other: &Token) -> bool {
        match other {
            Token::Keyword(k) => k == self,
            Token::Symbol(c) => c == self,
            Token::Identifier(_) => matches!(
                self,
                &TokenType::Name | &TokenType::Type | &TokenType::ReturnType
            ),
            Token::StringConstant(_) | Token::IntConstant(_) => self == &TokenType::Constant,
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::ClassVarDec => write!(f, "class var declaration"),
            TokenType::Constant => write!(f, "constant"),
            TokenType::Name => write!(f, "name"),
            TokenType::BinaryOp => write!(f, "binary op"),
            TokenType::UnaryOp => write!(f, "unary op"),
            TokenType::Statement => write!(f, "statement"),
            TokenType::SubroutineDec => write!(f, "subroutine declaration"),
            TokenType::Type => write!(f, "value type"),
            TokenType::ReturnType => write!(f, "return type"),
        }
    }
}

impl PartialEq<TokenType> for i16 {
    fn eq(&self, other: &TokenType) -> bool {
        other == &TokenType::Constant
    }
}
impl PartialEq<TokenType> for char {
    fn eq(&self, other: &TokenType) -> bool {
        match other {
            TokenType::BinaryOp => {
                matches!(
                    self,
                    '+' | '-' | '*' | '/' | '%' | '&' | '|' | '<' | '>' | '='
                )
            }
            TokenType::UnaryOp => matches!(self, '-' | '~'),
            _ => false,
        }
    }
}
impl PartialEq<TokenType> for Keyword {
    fn eq(&self, other: &TokenType) -> bool {
        match other {
            TokenType::Constant => matches!(self, True | False | This | Null),
            TokenType::ClassVarDec => matches!(self, Static | Field),
            TokenType::Statement => matches!(self, Let | If | While | Do | Return),
            TokenType::SubroutineDec => matches!(self, Constructor | Function | Method),
            TokenType::Type => matches!(self, Int | Char | Boolean),
            TokenType::ReturnType => matches!(self, Void | Int | Char | Boolean),
            _ => false,
        }
    }
}
impl PartialEq<Keyword> for TokenType {
    fn eq(&self, other: &Keyword) -> bool {
        match self {
            TokenType::Constant => matches!(other, True | False | This | Null),
            TokenType::ClassVarDec => matches!(other, Static | Field),
            TokenType::Statement => matches!(other, Let | If | While | Do | Return),
            TokenType::SubroutineDec => matches!(other, Constructor | Function | Method),
            TokenType::Type => matches!(other, Int | Char | Boolean),
            TokenType::ReturnType => matches!(other, Void | Int | Char | Boolean),
            _ => false,
        }
    }
}
impl PartialEq<TokenType> for String {
    fn eq(&self, other: &TokenType) -> bool {
        other == &TokenType::Constant
    }
}
