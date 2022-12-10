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
