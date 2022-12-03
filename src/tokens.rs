use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};
use Keyword::*;

use crate::token_type::TokenType;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Token {
    keyword: Option<Keyword>,
    symbol: Option<TokenWrapper>,
    identifier: Option<Identifier>,
    int_const: Option<TokenWrapper>,
    str_const: Option<TokenWrapper>,
}
impl ValidToken for Token {}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (
            &self.keyword,
            &self.symbol,
            &self.identifier,
            &self.int_const,
            &self.str_const,
        ) {
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
        match (
            &self.keyword,
            &self.symbol,
            &self.identifier,
            &self.int_const,
            &self.str_const,
        ) {
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
            symbol: Some(TokenWrapper::from(value)),
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
            int_const: Some(TokenWrapper::from(value)),
            ..Default::default()
        }
    }
}
impl From<String> for Token {
    fn from(value: String) -> Self {
        Token {
            str_const: Some(TokenWrapper::from(value)),
            ..Default::default()
        }
    }
}
impl PartialEq<char> for Token {
    fn eq(&self, other: &char) -> bool {
        if let Some(t) = &self.symbol {
            t == other
        } else {
            false
        }
    }
}
impl PartialEq<Keyword> for Token {
    fn eq(&self, other: &Keyword) -> bool {
        if let Some(t) = &self.keyword {
            other == t
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
            t == other
        } else {
            false
        }
    }
}
impl PartialEq<i16> for Token {
    fn eq(&self, other: &i16) -> bool {
        if let Some(t) = &self.int_const {
            t == other
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
        if let Some(TokenWrapper::Symbol(c)) = value.symbol {
            Ok(c)
        } else {
            Err(value)
        }
    }
}
impl TryFrom<Token> for Identifier {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if value.identifier.is_some() {
            Ok(value.identifier.unwrap())
        } else {
            Err(value)
        }
    }
}
impl TryFrom<Token> for i16 {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Some(TokenWrapper::IntConstant(i)) = value.str_const {
            Ok(i)
        } else {
            Err(value)
        }
    }
}
impl TryFrom<Token> for String {
    type Error = Token;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        if let Some(TokenWrapper::StringConstant(s)) = value.str_const {
            Ok(s)
        } else {
            Err(value)
        }
    }
}

impl Token {
    pub fn keyword(&self) -> Option<&Keyword> {
        self.keyword.as_ref()
    }
    pub fn symbol(&self) -> Option<&TokenWrapper> {
        self.symbol.as_ref()
    }
    pub fn identifier(&self) -> Option<&Identifier> {
        self.identifier.as_ref()
    }
    pub fn int_const(&self) -> Option<&TokenWrapper> {
        self.int_const.as_ref()
    }
    pub fn str_const(&self) -> Option<&TokenWrapper> {
        self.str_const.as_ref()
    }
}

impl PartialEq<Token> for char {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = &other.symbol {
            t == self
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
            t == self
        } else {
            false
        }
    }
}
impl PartialEq<Token> for i16 {
    fn eq(&self, other: &Token) -> bool {
        if let Some(t) = &other.int_const {
            t == self
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
            },
        }
    }
}
impl ValidToken for TokenWrapper {}
impl From<char> for TokenWrapper {
    fn from(value: char) -> Self {
        TokenWrapper::Symbol(value)
    }
}
impl From<i16> for TokenWrapper {
    fn from(value: i16) -> Self {
        TokenWrapper::IntConstant(value)
    }
}
impl From<String> for TokenWrapper {
    fn from(value: String) -> Self {
        TokenWrapper::StringConstant(value)
    }
}
impl PartialEq<char> for TokenWrapper {
    fn eq(&self, other: &char) -> bool {
        match self {
            Self::Symbol(c) => c == other,
            _ => false,
        }
    }
}
impl PartialEq<i16> for TokenWrapper {
    fn eq(&self, other: &i16) -> bool {
        match self {
            Self::IntConstant(i) => i == other,
            _ => false,
        }
    }
}impl PartialEq<String> for TokenWrapper {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::StringConstant(s) => s == other,
            _ => false,
        }
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
impl ValidToken for Keyword {}

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
