use std::{
    fmt::{Debug, Display},
    ops::Sub,
};

use crate::{
    names::{NameSet, Names},
    tokens::{
        Identifier,
        Keyword::{self, *},
        Token, TokenWrapper, ValidToken,
    },
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
        if let Some(t) = other.get_keyword() {
            t == self
        } else if let Some(t) = other.get_symbol() {
            t == self
        } else if let Some(t) = other.get_identifier() {
            t == self
        } else if let Some(t) = other.get_int_const() {
            t == self
        } else if let Some(t) = other.get_str_const() {
            t == self
        } else {
            false
        }
    }
}
impl TokenType {
    pub fn compare(t1: &dyn ValidToken, t2: &dyn ValidToken) -> bool {
        (t1 == &Self::ClassVarDec) == (t2 == &Self::ClassVarDec)
            && (t1 == &Self::Name) == (t2 == &Self::Name)
            && (t1 == &Self::Constant) == (t2 == &Self::Constant)
            && (t1 == &Self::BinaryOp) == (t2 == &Self::BinaryOp)
            && (t1 == &Self::UnaryOp) == (t2 == &Self::UnaryOp)
            && (t1 == &Self::Statement) == (t2 == &Self::Statement)
            && (t1 == &Self::SubroutineDec) == (t2 == &Self::SubroutineDec)
            && (t1 == &Self::Type) == (t2 == &Self::Type)
            && (t1 == &Self::ReturnType) == (t2 == &Self::ReturnType)
    }
}
impl crate::tokens::ValidToken for TokenType {}
// impl PartialEq<TokenWrapper> for TokenType {
//     fn eq(&self, other: &TokenWrapper) -> bool {
//         match (self, other) {
//             (Self::UnaryOp, TokenWrapper::Symbol(c)) => matches!(c, '-'|'~'),
//             (Self::BinaryOp, TokenWrapper::Symbol(c)) => matches!(c, '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '='),
//             (Self::Constant, _) => true,
//             _ => false,
//         }
//     }
// }

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
