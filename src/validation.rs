use std::fmt::{Display, Debug};

use crate::{
    tokens::{Keyword::{self, *}, TokenWrapper, Identifier}, names::{Names, NameSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    ClassVarDec,
    Constant,
    Name(Names),
    BinaryOp,
    UnaryOp,
    Statement,
    SubroutineDec,
    Type,
    ReturnType,
}

impl PartialEq<TokenWrapper> for TokenType {
    fn eq(&self, other: &TokenWrapper) -> bool {
        match (self, other) {
            (Self::UnaryOp, TokenWrapper::Symbol(c)) => matches!(c, '-'|'~'),
            (Self::BinaryOp, TokenWrapper::Symbol(c)) => matches!(c, '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='),
            (Self::Constant, _) => true,
            _ => false,
        }
    }
}
pub trait Token: Display + Debug + PartialEq<TokenType> {
    fn as_token(&self) -> Box<dyn Token>;
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
impl Token for TokenType {
    fn as_token(&self) -> Box<dyn Token> {
        Box::new(*self)
    }
}

impl Token for i16 {
    fn as_token(&self) -> Box<dyn Token> {
        Box::new(Into::<TokenWrapper>::into(*self))
    }
}
impl PartialEq<TokenType> for i16 {
    fn eq(&self, other: &TokenType) -> bool {
        other == &TokenType::Constant
    }
}


impl Token for char {
    fn as_token(&self) -> Box<dyn Token> {
        Box::new(Into::<TokenWrapper>::into(*self))
    }
}
impl PartialEq<TokenType> for char {
    fn eq(&self, other: &TokenType) -> bool {
        match other {
            TokenType::BinaryOp => matches!(self, '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='),
            TokenType::UnaryOp => matches!(self, '-' | '~'),
            _ => false,
        }
    }
}
impl Token for Keyword {
    fn as_token(&self) -> Box<dyn Token> {
        Box::new(*self)
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
            TokenType::ReturnType => matches!(self, Void | Int | Char |Boolean),
            _ => false,
        }
    }
}

impl Token for TokenWrapper {
    fn as_token(&self) -> Box<dyn Token> {
        Box::new(*self)
    }
}
impl PartialEq<TokenType> for TokenWrapper {
    fn eq(&self, other: &TokenType) -> bool {
        match (self, other) {
            (Self::Symbol(c), _) => c == other,
            (_, TokenType::Constant) => true,
        }
    }
}

impl Token for String {
    fn as_token(&self) -> Box<dyn Token> {
        Box::new(Into::<TokenWrapper>::into(*self))
    }
}
impl PartialEq<TokenType> for String {
    fn eq(&self, other: &TokenType) -> bool {
        other == &TokenType::Constant
    }
}

impl Token for Identifier {
    fn as_token(&self) -> Box<dyn Token> {
        Box::new(*self)
    }
}
impl PartialEq<TokenType> for Identifier {
    fn eq(&self, other: &TokenType) -> bool {
        matches!(other, TokenType::Name(_))
    }
}
