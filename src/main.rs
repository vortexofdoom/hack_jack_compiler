use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;

mod tokenizer;
mod tokens;
mod compilation_engine;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut files: Vec<PathBuf> = vec![];
    let file_path = Path::new(&args[1]);
    if file_path.is_dir() {
        for entry in file_path.read_dir().unwrap() {
            if let Some(x) = entry.as_ref().unwrap().path().extension() {
                if x.to_str().unwrap() == "jack" {
                    files.push(entry.as_ref().unwrap().path())
                }
            }
        }
    } else if let Some("jack") = file_path.extension().unwrap().to_str() {
        files.push(file_path.to_path_buf())
    }
    for file in files {
        let filename = file_path.file_stem().unwrap().to_str().unwrap();
        if let Ok(code) = fs::read_to_string(file) {
            let tokens = tokenizer::parse(code).expect("tokenizer error");
            let output = File::create(Path::new(filename).with_extension("xml"))
                .expect("failed to create file");
            let mut writer = BufWriter::new(output);
            for t in tokens.iter() {
                write!(writer, "{t}").expect("failed to write to file");
            }
            writer.flush().unwrap();
        }
    }
}
