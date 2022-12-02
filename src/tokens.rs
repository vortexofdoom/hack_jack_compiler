use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Debug},
};
use Keyword::*;

use crate::validation::{Token, TokenType};

// #[derive(Default, PartialEq, Eq)]
// pub struct Token {
//     keyword: Option<Keyword>,
//     symbol: Option<char>,
//     identifier: Option<Identifier>,
//     int_const: Option<i16>,
//     str_const: Option<String>,
// }

impl PartialEq<TokenType> for Box<dyn Token> {
    fn eq(&self, other: &TokenType) -> bool {
        self.as_ref() == other
    }
}

// impl From<Identifier> for Token {
//     fn from(value: Identifier) -> Self {

//     }
// }

// impl From<char> for Token {
//     fn from(value: char) -> Self {
//         Token {
//             symbol: Some(value),
//             ..Default::default()
//         }
//     }
// }

// impl From<Keyword> for Token {
//     fn from(value: Keyword) -> Self {
//         Token {
//             keyword: Some(value),
//             ..Default::default()
//         }
//     }
// }

// impl From<i16> for Token {
//     fn from(value: i16) -> Self {
//         Token {
//             int_const: Some(value),
//             ..Default::default()
//         }
//     }
// }

// impl From<String> for Token {
//     fn from(value: String) -> Self {
//         Token {
//             str_const: Some(value),
//             ..Default::default()
//         }
//     }
// }

// impl Token {
//     pub fn get(&self) -> Box<dyn ValidToken> {
//         match (self.keyword, self.symbol, self.identifier, self.int_const, self.str_const) {
//             (Some(t),None,None,None,None) => self.get_keyword(),
//             (None,Some(t),None,None,None) => self.get_symbol(),
//             (None,None,Some(t),None,None) => self.get_identifier(),
//             (None,None,None,Some(t),None) => self.get_int_const(),
//             (None,None,None,None,Some(t)) => self.get_str_const(),
//             _ => ,
//         }
//     }
//     fn get_keyword(&self) -> Keyword {
//         self.keyword.unwrap()
//     }
//     fn get_symbol(&self) -> TokenWrapper {
//         self.symbol.unwrap().into()
//     }
//     fn get_identifier(&self) -> Identifier {
//         self.identifier.unwrap()
//     }
//     fn get_int_const(&self) -> TokenWrapper {
//         self.int_const.unwrap().into()
//     }
//     fn get_str_const(&self) -> TokenWrapper {
//         self.str_const.unwrap().into()
//     }
//     pub fn keyword(k: Keyword) -> Self {
//         Token {
//             keyword: Some(k),
//             ..Default::default()
//         }
//     }

//     pub fn symbol(c: char) -> Self {
//         Token {
//             symbol: Some(c),
//             ..Default::default()
//         }
//     }

//     pub fn identifier(s: String) -> Self {
//         Token {
//             identifier: Some(Identifier(s)),
//             ..Default::default()
//         }
//     }

//     pub fn int_const(i: i16) -> Self {
//         Token {
//             int_const: Some(i),
//             ..Default::default()
//         }
//     }

//     pub fn str_const(s: String) -> Self {
//         Token {
//             str_const: Some(s),
//             ..Default::default()
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub enum TokenWrapper {
    Symbol(char),
    IntConstant(i16),
    StringConstant(String),
}

impl Display for TokenWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenWrapper::StringConstant(s) => write!(f, "<stringConstant>{s}</stringConstant>"),
            TokenWrapper::IntConstant(i) => write!(f, "<integerConstant>{i}</integerConstant>"),
            TokenWrapper::Symbol(c) => match c {
                '<' => write!(f, "<symbol>&lt;</symbol>"),
                '>' => write!(f, "<symbol>&gt;</symbol>"),
                '"' => write!(f, "<symbol>&quot;</symbol>"),
                '&' => write!(f, "<symbol>&amp;</symbol>"),
                _ => write!(f, "<symbol>{c}</symbol>"),
            }
            _ => write!(f, ""),
        }
    }
}

impl TokenWrapper {
    pub fn wrap<T: Into<TokenWrapper>>(token: T) -> Self {
        Into::into(token)
    }
}

// impl TokenWrapper {
//     fn try_from(t: &dyn Token) -> Option<Self> {
//         if let w = Self::from(t as &Any) {
//             Some(w)
//         } else {
//             None
//         }
//     }
// }

// impl From<i16> for TokenWrapper {
//     fn from(i: i16) -> Self {
//         Self::IntConstant(i)
//     }
// }

// impl From<char> for TokenWrapper {
//     fn from(c: char) -> Self {
//         Self::Symbol(c)
//     }
// }
// impl From<String> for TokenWrapper {
//     fn from(s: String) -> Self {
//         Self::StringConstant(s)
//     }
// }

impl Into<TokenWrapper> for i16 {
    fn into(self) -> TokenWrapper {
        TokenWrapper::IntConstant(self)
    }
}

impl Into<TokenWrapper> for char {
    fn into(self) -> TokenWrapper {
        TokenWrapper::Symbol(self)
    }
}
impl Into<TokenWrapper> for String {
    fn into(self) -> TokenWrapper {
        TokenWrapper::StringConstant(self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier(pub String);
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<identifier>{}</identifier>", self.0)
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
        write!(f, "<keyword>{kw}</keyword>")
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
        hs
    };
}
