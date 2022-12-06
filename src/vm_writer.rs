use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path, fmt::Display,
};

use crate::token_type::TokenType;

pub trait CodeWriter:  Default {
    fn write(&mut self, contents: impl Display);
    fn flush(&mut self);
    fn start(&mut self, code: &str);
    fn finish(&mut self, code: &str);
    fn new(filename: &str) -> Self;
}

#[derive(Default)]
pub struct VMWriter {
    writer: Option<BufWriter<File>>,
}

impl CodeWriter for VMWriter {
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
        VMWriter {
            writer: Some(writer),
        }
    }

    fn write(&mut self, contents: impl Display) {
        write!(self.writer.as_mut().expect("no writer"), "{contents}").expect("failed to write");
    }

    fn flush(&mut self) {
        self.writer.as_mut().expect("no writer").flush().unwrap();
    }
}

impl VMWriter {
}
