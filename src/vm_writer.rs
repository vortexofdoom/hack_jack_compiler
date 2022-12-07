use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{token_type::TokenType, symbol_table::{SymbolTable, Kind}, tokens::{Token, Keyword::*}};

// Same as VMTranslator enum
// Someday I want to combine the Compiler/VM Translator/Assembler
pub enum VmCommand<'a> {
    // Arithmetic
    Add,
    Sub,
    Neg,
    Compare(Comparison),
    And,
    Or,
    Not,
    //mem access
    Push(MemSegment, u16),
    Pop(MemSegment, u16),
    // Branching
    Label(&'a str),
    Goto(&'a str),
    IfGoto(&'a str),
    // Function
    Function(&'a str, u16),
    Call(&'a str, u16),
    Return,
}

pub enum MemSegment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

pub enum Comparison {
    Eq,
    GT,
    LT,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::GT => write!(f, "gt"),
            Self::LT => write!(f, "lt"),
        }
    }
}

impl Display for MemSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Argument => write!(f, "argument"),
            Self::This => write!(f, "this"),
            Self::That => write!(f, "that"),
            Self::Constant => write!(f, "constant"),
            Self::Static => write!(f, "static"),
            Self::Pointer => write!(f, "pointer"),
            Self::Temp => write!(f, "temp"),
        }
    }
}

impl Display for VmCommand<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VmCommand::Add => write!(f, "add"),
            VmCommand::Sub => write!(f, "sub"),
            VmCommand::Neg => write!(f, "neg"),
            VmCommand::Compare(cmp) => write!(f, "{cmp}"),
            VmCommand::And => write!(f, "and"),
            VmCommand::Or => write!(f, "or"),
            VmCommand::Not => write!(f, "not"),
            VmCommand::Push(seg, arg) => write!(f, "push {seg} {arg}"),
            VmCommand::Pop(seg, arg) => write!(f, "pop {seg} {arg}"),
            VmCommand::Label(label) => write!(f, "label {label}"),
            VmCommand::Goto(label) => write!(f, "goto {label}"),
            VmCommand::IfGoto(label) => write!(f, "if-goto {label}"),
            VmCommand::Function(func, n) => write!(f, "function {func} {n}"),
            VmCommand::Call(func, n) => write!(f, "call {func} {n}"),
            VmCommand::Return => write!(f, "return"),
        }
    }
}

pub trait CodeWriter: Default {
    fn write(&mut self, contents: impl Display);
    fn flush(&mut self);
    fn start(&mut self, code: &str);
    fn finish(&mut self, code: &str);
    fn new(filename: &str) -> Self;
}

#[derive(Default)]
pub struct VmWriter {
    writer: Option<BufWriter<File>>,
    symbol_table: SymbolTable,
}

impl CodeWriter for VmWriter {
    fn start(&mut self, code: &str) {
        todo!()
    }

    fn finish(&mut self, code: &str) {
        todo!()
    }
    fn new(filename: &str) -> Self {
        let file =
            File::create(Path::new(filename).with_extension("vm")).expect("could not create file");
        let writer = BufWriter::new(file);
        VmWriter {
            writer: Some(writer),
            symbol_table: SymbolTable::default()
        }
    }

    fn write(&mut self, contents: impl Display) {
        write!(self.writer.as_mut().expect("no writer"), "{contents}").expect("failed to write");
    }

    fn flush(&mut self) {
        self.writer.as_mut().expect("no writer").flush().unwrap();
    }
}

impl VmWriter {
    pub fn compile_var(&mut self, kind: Kind, typ: Token, name: String) {
        let type_of = match typ {
            Token::Keyword(k @ (Int | Char | Boolean)) => format!("{k}"),
            Token::Identifier(s) => s,
            _ => String::from("shouldn't be any other var type here"),
        };
        self.symbol_table
                .define(kind, &type_of, name)
                .map_err(|e| /*self.throw_error(e)*/{})
                .unwrap();
    }

    pub fn start_subroutine(&self) {
        self.symbol_table.start_subroutine();
    }
}
