use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};
use Keyword::*;

use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Symbol(char),
    Identifier(String),
    IntConstant(i16),
    StringConstant(String),
}

impl Token {
    pub fn as_type(&self) -> String {
        match self {
            Token::Keyword(k @ (Int | Char | Boolean)) => format!("{k}"),
            Token::Identifier(s) => s.clone(),
            _ => String::from("invalid type"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "<keyword> {k} </keyword>"),
            Token::Identifier(s) => write!(f, "<identifier> {s} </identifier>"),
            Token::StringConstant(s) => write!(f, "<stringConstant> {s} </stringConstant>"),
            Token::IntConstant(i) => write!(f, "<integerConstant> {i} </integerConstant>"),
            Token::Symbol(c) => match c {
                '<' => write!(f, "<symbol> &lt; </symbol>"),
                '>' => write!(f, "<symbol> &gt; </symbol>"),
                '"' => write!(f, "<symbol> &quot; </symbol>"),
                '&' => write!(f, "<symbol> &amp; </symbol>"),
                _ => write!(f, "<symbol> {c} </symbol>"),
            },
        }
    }
}
impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        match self {
            Token::Keyword(t) => t == other,
            Token::Symbol(t) => t == other,
            Token::Identifier(t) => t == other,
            Token::IntConstant(t) => t == other,
            Token::StringConstant(t) => t == other,
        }
    }
}
impl PartialEq<Option<Token>> for Token {
    fn eq(&self, other: &Option<Token>) -> bool {
        match (self, other) {
            (Self::Keyword(l0), Some(Self::Keyword(r0))) => l0 == r0,
            (Self::Symbol(l0), Some(Self::Symbol(r0))) => l0 == r0,
            (Self::Identifier(l0), Some(Self::Identifier(r0))) => l0 == r0,
            (Self::IntConstant(l0), Some(Self::IntConstant(r0))) => l0 == r0,
            (Self::StringConstant(l0), Some(Self::StringConstant(r0))) => l0 == r0,
            _ => false,
        }
    }
}

// TODO: Consider macro use here
impl PartialEq<char> for Token {
    fn eq(&self, other: &char) -> bool {
        if let Self::Symbol(t) = &self {
            t == other
        } else {
            false
        }
    }
}
impl PartialEq<Keyword> for Token {
    fn eq(&self, other: &Keyword) -> bool {
        if let Self::Keyword(t) = &self {
            t == other
        } else {
            false
        }
    }
}
impl PartialEq<String> for Token {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::Identifier(s) | Self::StringConstant(s) => s == other,
            _ => false,
        }
    }
}
impl PartialEq<i16> for Token {
    fn eq(&self, other: &i16) -> bool {
        if let Self::IntConstant(t) = &self {
            t == other
        } else {
            false
        }
    }
}
impl PartialEq<Token> for char {
    fn eq(&self, other: &Token) -> bool {
        other == self
    }
}
impl PartialEq<Token> for Keyword {
    fn eq(&self, other: &Token) -> bool {
        other == self
    }
}
impl PartialEq<Token> for String {
    fn eq(&self, other: &Token) -> bool {
        other == self
    }
}
impl PartialEq<Token> for i16 {
    fn eq(&self, other: &Token) -> bool {
        other == self
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kw = match self {
            Class => "class",
            Constructor => "constructor",
            Function => "function",
            Method => "method",
            Field => "field",
            Static => "static",
            Var => "var",
            Int => "int",
            Char => "char",
            Boolean => "boolean",
            Void => "void",
            True => "true",
            False => "false",
            Null => "null",
            This => "this",
            Let => "let",
            Do => "do",
            If => "if",
            Else => "else",
            While => "while",
            Return => "return",
        };
        write!(f, "{kw}")
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut hm = HashMap::new();
        hm.insert("class", Class);
        hm.insert("constructor", Constructor);
        hm.insert("function", Function);
        hm.insert("method", Method);
        hm.insert("field", Field);
        hm.insert("static", Static);
        hm.insert("var", Var);
        hm.insert("int", Int);
        hm.insert("char", Char);
        hm.insert("boolean", Boolean);
        hm.insert("void", Void);
        hm.insert("true", True);
        hm.insert("false", False);
        hm.insert("null", Null);
        hm.insert("this", This);
        hm.insert("let", Let);
        hm.insert("do", Do);
        hm.insert("if", If);
        hm.insert("else", Else);
        hm.insert("while", While);
        hm.insert("return", Return);
        hm
    };
    pub static ref SYMBOLS: HashSet<char> = {
        let mut hs = HashSet::new();
        hs.insert('{');
        hs.insert('}');
        hs.insert('(');
        hs.insert(')');
        hs.insert('[');
        hs.insert(']');
        hs.insert('.');
        hs.insert(',');
        hs.insert(';');
        hs.insert('+');
        hs.insert('-');
        hs.insert('*');
        hs.insert('/');
        hs.insert('&');
        hs.insert('|');
        hs.insert('<');
        hs.insert('>');
        hs.insert('=');
        hs.insert('~');
        hs.insert('"');
        hs.insert('%');
        hs
    };
}
