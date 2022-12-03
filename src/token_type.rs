use std::fmt::{Debug, Display};

use crate::tokens::{
    Identifier,
    Keyword::{self, *},
    Token, TokenWrapper,
};

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
impl PartialEq<Token> for TokenType {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = other.keyword() {
            t == self
        } else if let Some(t) = other.symbol() {
            t == self
        } else if let Some(t) = other.identifier() {
            t == self
        } else if let Some(t) = other.int_const() {
            t == self
        } else if let Some(t) = other.str_const() {
            t == self
        } else {
            false
        }
    }
}

impl crate::tokens::ValidToken for TokenType {}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
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
                matches!(self, '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=')
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
impl PartialEq<TokenType> for TokenWrapper {
    fn eq(&self, other: &TokenType) -> bool {
        match (self, other) {
            (Self::Symbol(c), _) => c == other,
            (_, TokenType::Constant) => true,
            _ => false,
        }
    }
}
impl PartialEq<TokenType> for String {
    fn eq(&self, other: &TokenType) -> bool {
        other == &TokenType::Constant
    }
}
impl PartialEq<TokenType> for Identifier {
    fn eq(&self, other: &TokenType) -> bool {
        matches!(other, TokenType::Name)
    }
}
