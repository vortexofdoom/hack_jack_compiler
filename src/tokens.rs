use std::{collections::{HashSet, HashMap}, fmt::Display};
use Keyword::*;

#[derive(Clone)]
pub enum Token {
    Keyword(Keyword),
    Symbol(char),
    Identifier(String),
    IntConst(i16),
    StringConst(String),
}

#[derive(Clone, Copy)]
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(k) => write!(f, "<keyword>{k}</keyword>"),
            Token::Symbol(c) => {
                match c {
                    '<' => write!(f, "<symbol>&lt;</symbol>"),
                    '>' => write!(f, "<symbol>&gt;</symbol>"),
                    '"' => write!(f, "<symbol>&quot;</symbol>"),
                    '&' => write!(f, "<symbol>&amp;</symbol>"),
                    _ => write!(f, "<symbol>{c}</symbol>"),
                }
            },
            Token::Identifier(s) => write!(f, "<identifier>{s}</identifier>"),
            Token::IntConst(i) => write!(f, "<integerConstant>{i}</integerConstant>"),
            Token::StringConst(s) => write!(f, "<stringConstant>{s}</stringConstant>"),
        }
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
        hs
    };
}