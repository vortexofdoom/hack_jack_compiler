use std::{fs::File, path::Path, io::{BufWriter,Write}, fmt::Display};

#[derive(Default)]
pub struct XMLWriter {
    writer: Option<BufWriter<File>>,
    tag: String,
}

impl XMLWriter {
    pub fn new(filename: &str) -> Self {
        let file = File::create(Path::new(filename).with_extension("xml"))
            .expect("could not create file");
        let writer = BufWriter::new(file);
        XMLWriter { writer: Some(writer), tag: String::new() }
    }

    pub fn open_tag(&mut self, tag: &str) {
        self.tag = String::from(tag);
        writeln!(self.writer.as_mut().expect("no writer"),"<{tag}>").expect("failed to write tag");
        self.writer.as_mut().unwrap().flush().unwrap();
    }
    pub fn close_tag(&mut self, tag: &str) {
        writeln!(self.writer.as_mut().expect("no writer"),"</{tag}>").expect("failed to write tag");
        self.writer.as_mut().unwrap().flush().unwrap();
    }
    pub fn write<T: Display>(&mut self, contents: T) {
        writeln!(self.writer.as_mut().expect("no writer"), "{contents}").expect("failed to write");
        self.writer.as_mut().unwrap().flush().unwrap();
    }
}