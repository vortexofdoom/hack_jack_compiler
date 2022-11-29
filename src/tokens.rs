use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};
use Keyword::*;

#[derive(Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(char),
    Identifier(String),
    IntConst(i16),
    StringConst(String),
}

impl Token {
    pub fn is_valid(&self, token_type: TokenType) -> bool {
        match (self, token_type) {
            (Token::Keyword(Field), TokenType::ClassVar)
            | (Token::Keyword(Static), TokenType::ClassVar) => true,

            (Token::Symbol('+'), TokenType::Op)
            | (Token::Symbol('-'), TokenType::Op)
            | (Token::Symbol('*'), TokenType::Op)
            | (Token::Symbol('/'), TokenType::Op)
            | (Token::Symbol('&'), TokenType::Op)
            | (Token::Symbol('|'), TokenType::Op)
            | (Token::Symbol('<'), TokenType::Op)
            | (Token::Symbol('>'), TokenType::Op)
            | (Token::Symbol('='), TokenType::Op)  => true,

            (Token::Symbol('-'), TokenType::UnaryOp)
            | (Token::Symbol('~'), TokenType::UnaryOp) => true,

            (Token::Keyword(True), TokenType::Constant)
            | (Token::Keyword(False), TokenType::Constant)
            | (Token::Keyword(Null), TokenType::Constant)
            | (Token::Keyword(This), TokenType::Constant)
            | (Token::StringConst(_), TokenType::Constant)
            | (Token::IntConst(_), TokenType::Constant) => true,

            (Token::Keyword(Let), TokenType::Statement)
            | (Token::Keyword(While), TokenType::Statement)
            | (Token::Keyword(Do), TokenType::Statement)
            | (Token::Keyword(If), TokenType::Statement)
            | (Token::Keyword(Return), TokenType::Statement) => true,

            (Token::Keyword(Int), TokenType::Type)
            | (Token::Keyword(Char), TokenType::Type)
            | (Token::Keyword(Boolean), TokenType::Type)
            | (Token::Identifier(_), TokenType::Type) => true,

            (Token::Keyword(Constructor), TokenType::Subroutine)
            | (Token::Keyword(Function), TokenType::Subroutine)
            | (Token::Keyword(Method), TokenType::Subroutine) => true,

            (Token::Identifier(_), TokenType::VarName) 
            | (Token::Identifier(_), TokenType::ClassName)
            | (Token::Identifier(_), TokenType::SubroutineName) => true,

            _ => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
    ClassName,
    ClassVar,
    Constant,
    Statement,
    Subroutine,
    SubroutineName,
    Term,
    Type,
    Var,
    VarName,
    Op,
    UnaryOp,
    Other,
}

#[derive(Clone, Copy, PartialEq)]
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
            Token::Symbol(c) => match c {
                '<' => write!(f, "<symbol>&lt;</symbol>"),
                '>' => write!(f, "<symbol>&gt;</symbol>"),
                '"' => write!(f, "<symbol>&quot;</symbol>"),
                '&' => write!(f, "<symbol>&amp;</symbol>"),
                _ => write!(f, "<symbol>{c}</symbol>"),
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
