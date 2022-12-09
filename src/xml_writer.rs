use std::{
    fmt::Display,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::vm_writer::CodeWriter;

#[derive(Default)]
pub struct XMLWriter {
    writer: Option<BufWriter<File>>,
}
impl CodeWriter for XMLWriter {
    // fn start(&mut self, code: &str) {
    //     writeln!(self.writer.as_mut().unwrap(), "<{code}>").expect("failed to write tag");
    //     self.flush();
    // }
    // fn finish(&mut self, tag: &str) {
    //     writeln!(self.writer.as_mut().unwrap(), "</{tag}>").expect("failed to write tag");
    //     self.flush();
    // }
    fn new(filename: &str) -> Self {
        let file =
            File::create(Path::new(filename).with_extension("xml")).expect("could not create file");
        let writer = BufWriter::new(file);
        XMLWriter {
            writer: Some(writer),
        }
    }

    fn write(&mut self, contents: impl Display) {
        writeln!(self.writer.as_mut().unwrap(), "{contents}").expect("failed to write");
        self.flush();
    }

    fn flush(&mut self) {
        self.writer.as_mut().unwrap().flush().unwrap();
    }
}

impl XMLWriter {}
