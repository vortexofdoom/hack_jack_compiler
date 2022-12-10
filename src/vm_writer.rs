use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::tokens::{Keyword::*, Token};

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
    Push(MemSegment, i16),
    Pop(MemSegment, i16),
    // Branching
    Label(&'a str),
    Goto(&'a str),
    IfGoto(&'a str),
    // Function
    Function(&'a str, i16),
    Call(&'a str, i16),
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn new(filename: &str) -> Self;
}

#[derive(Default)]
pub struct VmWriter {
    writer: Option<BufWriter<File>>,
    if_counter: u16,
    while_counter: u16,
}

impl CodeWriter for VmWriter {
    fn new(filename: &str) -> Self {
        let file =
            File::create(Path::new(filename).with_extension("vm")).expect("could not create file");
        let writer = BufWriter::new(file);
        VmWriter {
            writer: Some(writer),
            if_counter: 0,
            while_counter: 0,
        }
    }

    fn write(&mut self, contents: impl Display) {
        writeln!(self.writer.as_mut().expect("no writer"), "{contents}").expect("failed to write");
    }

    fn flush(&mut self) {
        self.writer.as_mut().expect("no writer").flush().unwrap();
    }
}

impl VmWriter {
    pub fn generate_label(&mut self, label: &str) -> String {
        let counter = if label == "if" {
            &mut self.if_counter
        } else {
            &mut self.while_counter
        };
        let label = format!("{label}{counter}");
        *counter += 1;
        label
    }

    pub fn write_constant(&mut self, t: Token) {
        match t {
            Token::Keyword(True) => {
                self.write(VmCommand::Push(MemSegment::Constant, 1));
                self.write(VmCommand::Neg);
            }
            Token::Keyword(False) | Token::Keyword(Null) => {
                self.write(VmCommand::Push(MemSegment::Constant, 0))
            }
            Token::Keyword(This) => self.write(VmCommand::Push(MemSegment::Pointer, 0)),
            Token::IntConstant(i) => self.write(VmCommand::Push(MemSegment::Constant, i)),
            Token::StringConstant(s) => {
                self.write(VmCommand::Push(MemSegment::Constant, s.len() as i16));
                self.write(VmCommand::Call("String.new", 1));
                for c in s.chars() {
                    self.write(VmCommand::Push(MemSegment::Constant, c as i16));
                    self.write(VmCommand::Call("String.appendChar", 2));
                }
            }
            _ => { /*only passing constants*/ }
        }
    }
}
