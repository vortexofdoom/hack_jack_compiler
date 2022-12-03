use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Debug},
};
use Keyword::*;

use crate::validation::TokenType;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Token {
    keyword: Option<Keyword>,
    symbol: Option<char>,
    identifier: Option<Identifier>,
    int_const: Option<i16>,
    str_const: Option<String>,
}
impl ValidToken for Token {}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.keyword, &self.symbol, &self.identifier, &self.int_const, &self.str_const) {
            (Some(t), None, None, None, None) => write!(f, "{t}"),
            (None, Some(t), None, None, None) => write!(f, "{t}"),
            (None, None, Some(t), None, None) => write!(f, "{t}"),
            (None, None, None, Some(t), None) => write!(f, "{t}"),
            (None, None, None, None, Some(t)) => write!(f, "{t}"),
            _ => Err(std::fmt::Error),
        }
    }
}
impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        match (&self.keyword, &self.symbol, &self.identifier, &self.int_const, &self.str_const) {
            (Some(t), None, None, None, None) => t == other,
            (None, Some(t), None, None, None) => t == other,
            (None, None, Some(t), None, None) => t == other,
            (None, None, None, Some(t), None) => t == other,
            (None, None, None, None, Some(t)) => t == other,
            _ => false,
        }
    }
}
impl From<Identifier> for Token {
    fn from(value: Identifier) -> Self {
        Token {
            identifier: Some(value),
            ..Default::default()
        }
    }
}
impl From<char> for Token {
    fn from(value: char) -> Self {
        Token {
            symbol: Some(value),
            ..Default::default()
        }
    }
}
impl From<Keyword> for Token {
    fn from(value: Keyword) -> Self {
        Token {
            keyword: Some(value),
            ..Default::default()
        }
    }
}
impl From<i16> for Token {
    fn from(value: i16) -> Self {
        Token {
            int_const: Some(value),
            ..Default::default()
        }
    }
}
impl From<String> for Token {
    fn from(value: String) -> Self {
        Token {
            str_const: Some(value),
            ..Default::default()
        }
    }
}
impl PartialEq<char> for Token {
    fn eq(&self, other: &char) -> bool {
        if let Some(t) = self.symbol {
            other == &t
        } else {
            false
        }
    }
}
impl PartialEq<Keyword> for Token {
    fn eq(&self, other: &Keyword) -> bool {
        if let Some(t) = self.keyword {
            other == &t
        } else {
            false
        }
    }
}
impl PartialEq<Identifier> for Token {
    fn eq(&self, other: &Identifier) -> bool {
        if let Some(t) = &self.identifier {
            other == t
        } else {
            false
        }
    }
}
impl PartialEq<String> for Token {
    fn eq(&self, other: &String) -> bool {
        if let Some(t) = &self.str_const {
            other == t
        } else {
            false
        }
    }
}
impl PartialEq<i16> for Token {
    fn eq(&self, other: &i16) -> bool {
        if let Some(t) = self.int_const {
            other == &t
        } else {
            false
        }
    }
}

impl TryFrom<Token> for Keyword {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Some(t) = value.keyword {
            Ok(t)
        } else {
            Err(value)
        }
    }
}
impl TryFrom<Token> for char {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Some(t) = value.symbol {
            Ok(t)
        } else {
            Err(value)
        }
    }
}
impl TryFrom<Token> for Identifier {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Some(_) = value.identifier {
            Ok(value.identifier.unwrap())
        } else {
            Err(value)
        }
    }
}
impl TryFrom<Token> for i16 {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Some(t) = value.int_const {
            Ok(t)
        } else {
            Err(value)
        }
    }
}
impl TryFrom<Token> for String {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Some(_) = value.str_const {
            Ok(value.str_const.unwrap())
        } else {
            Err(value)
        }
    }
}

impl Token {
    // pub fn get<F: FnOnce()>(&self) -> F {
    //     match (self.keyword, self.symbol, self.identifier, self.int_const, self.str_const) {
    //         (Some(t),None,None,None,None) => self.get_keyword(),
    //         (None,Some(t),None,None,None) => {self.get_symbol();}
    //         (None,None,Some(t),None,None) => {self.get_identifier();}
    //         (None,None,None,Some(t),None) => {self.get_int_const();}
    //         (None,None,None,None,Some(t)) => {self.get_str_const();}
    //         _ => {}
    //     }
    // }
    pub fn get_keyword(&self) -> Option<&Keyword> {
        self.keyword.as_ref()
    }
    pub fn get_symbol(&self) -> Option<&char> {
        self.symbol.as_ref()
    }
    pub fn get_identifier(&self) -> Option<&Identifier> {
        self.identifier.as_ref()
    }
    pub fn get_int_const(&self) -> Option<&i16> {
        self.int_const.as_ref()
    }
    pub fn get_str_const(&self) -> Option<&String> {
        self.str_const.as_ref()
    }
    pub fn keyword(k: Keyword) -> Self {
        Token {
            keyword: Some(k),
            ..Default::default()
        }
    }

    pub fn symbol(c: char) -> Self {
        Token {
            symbol: Some(c),
            ..Default::default()
        }
    }

    pub fn identifier(s: String) -> Self {
        Token {
            identifier: Some(Identifier(s)),
            ..Default::default()
        }
    }

    pub fn int_const(i: i16) -> Self {
        Token {
            int_const: Some(i),
            ..Default::default()
        }
    }

    pub fn str_const(s: String) -> Self {
        Token {
            str_const: Some(s),
            ..Default::default()
        }
    }
}

// impl Token {
//     fn from<T: ValidToken> (token: T) -> Self {
        
//     }
//     fn get_keyword(&self) -> Option<Keyword> {
//         self.keyword
//     }
//     fn get_symbol(&self) -> Option<char> {
//         self.symbol
//     }
//     fn get_identifier(&self) -> Option<Identifier> {
//         self.identifier
//     }
//     fn get_int_const(&self) -> Option<i16> {
//         self.int_const
//     }
//     fn get_str_const(&self) -> Option<String> {
//         self.str_const
//     }
// }
impl PartialEq<Token> for char {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = other.symbol {
            self == &t
        } else {
            false
        }
    }
}
impl PartialEq<Token> for Keyword {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = other.keyword {
            self == &t
        } else {
            false
        }
    }
}
impl PartialEq<Token> for Identifier {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = &other.identifier {
            self == t
        } else {
            false
        }
    }
}
impl PartialEq<Token> for String {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = &other.str_const {
            self == t
        } else {
            false
        }
    }
}
impl PartialEq<Token> for i16 {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = other.int_const {
            self == &t
        } else {
            false
        }
    }
}

pub trait ValidToken: Display + Debug + PartialEq<TokenType> {}
impl PartialEq<TokenType> for Box<dyn ValidToken> {
    fn eq(&self, other: &TokenType) -> bool {
        self.as_ref() == other
    }
}

// impl Token for TokenType { // this will never get used
// 
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
impl ValidToken for TokenWrapper {}
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
impl ValidToken for char {}
impl ValidToken for i16 {}
impl ValidToken for String {}

#[derive(Debug, PartialEq, Eq)]
pub struct Identifier(pub String);
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<identifier>{}</identifier>", self.0)
    }
}
impl ValidToken for Identifier {}
// impl Token for Identifier {
//     fn as_token(&self) -> Box<dyn Token> {
//         Box::new(*self)
//     }
// }

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
impl ValidToken for Keyword {
    // fn as_token(&self) -> Box<dyn Token> {
    //     Box::new(*self)
    // }
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
