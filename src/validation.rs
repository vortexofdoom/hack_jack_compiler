// Huge mess of TryFrom and other validation

use crate::{
    names::*,
    tokens::{
        Keyword::{self, *},
        Token,
    },
};

impl TryFrom<Box<Token>> for Keyword {
    type Error = Box<Token>;
    fn try_from(value: Box<Token>) -> Result<Self, Self::Error> {
        match *value {
            Token::Keyword(k) => Ok(k),
            _ => Err(value),
        }
    }
}
impl TryFrom<Box<Token>> for char {
    type Error = Box<Token>;
    fn try_from(value: Box<Token>) -> Result<Self, Self::Error> {
        match *value {
            Token::Symbol(c) => Ok(c),
            _ => Err(value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    ClassVarDec,
    Constant,
    Name(NameSet),
    BinaryOp,
    UnaryOp,
    Statement,
    SubroutineDec,
    Type,
}

pub trait ValidToken {
    fn is_valid_token_type(&self, token_type: TokenType) -> bool {
        (*self).is_valid_token_type(token_type)
    }
}

impl ValidToken for TokenType {
    fn is_valid_token_type(&self, token_type: TokenType) -> bool {
        matches!(self, token_type)
    }
}

impl ValidToken for char {
    fn is_valid_token_type(&self, token_type: TokenType) -> bool {
        match token_type {
            TokenType::UnaryOp => matches!(self, '-' | '~'),
            TokenType::BinaryOp => {
                matches!(self, '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=')
            }
            _ => false,
        }
    }
}

impl ValidToken for Keyword {
    fn is_valid_token_type(&self, token_type: TokenType) -> bool {
        match token_type {
            TokenType::Constant => matches!(self, True | False | This | Null),
            TokenType::ClassVarDec => matches!(self, Static | Field),
            TokenType::Statement => matches!(self, Let | If | While | Do | Return),
            TokenType::SubroutineDec => matches!(self, Constructor | Function | Method),
            TokenType::Type => matches!(self, Int | Char | Boolean),
            _ => false,
        }
    }
}

impl TryFrom<Keyword> for TokenType {
    type Error = Keyword;

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            True    => Ok(Self::Constant),
            False   => Ok(Self::Constant),
            This    => Ok(Self::Constant),
            Null    => Ok(Self::Constant),

            Static  => Ok(Self::ClassVarDec),
            Field   => Ok(Self::ClassVarDec),

            Constructor => Ok(Self::SubroutineDec),
            Function    => Ok(Self::SubroutineDec),
            Method      => Ok(Self::SubroutineDec),

            Int     => Ok(Self::Type),
            Char    => Ok(Self::Type),
            Boolean => Ok(Self::Type),

            Let => Ok(Self::Statement),
            If => Ok(Self::Statement),
            While => Ok(Self::Statement),
            Do => Ok(Self::Statement),
            Return => Ok(Self::Statement),
            _ => Err(value)
        }
    }
}

impl TryFrom<char> for TokenType {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' | '-' | '*' | '/' | '&' | '|' | '<' | '>' | '=' => Ok(Self::BinaryOp),
            '~' => Ok(Self::UnaryOp),
            _ => Err(value),
        }
    }
}

impl From<Keyword> for Token {
    fn from(k: Keyword) -> Self {
        Self::Keyword(k)
    }
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        Self::Symbol(c)
    }
}

impl TryFrom<Box<Token>> for TokenType {
    type Error = Box<Token>;

    fn try_from(value: Box<Token>) -> Result<Self, Self::Error> {
        match *value {
            Token::Keyword(k) => Ok(Self::try_from(k).map_err(|_| value)?),
            Token::Symbol(c) => Ok(Self::try_from(c).map_err(|_| value)?),
            Token::IntConst(_) => Ok(Self::Constant),
            Token::StringConst(_) => Ok(Self::Constant),
            _ => Err(value)
        }
    }
}