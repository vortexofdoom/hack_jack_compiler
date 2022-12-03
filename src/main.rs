use compilation_engine::CompilationEngine;
use std::path::{Path, PathBuf};
use tokenizer::Tokenizer;

#[macro_use]
extern crate lazy_static;

mod compilation_engine;
mod names;
mod tokenizer;
mod tokens;
mod token_type;

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
        let tknzr = Tokenizer::new(&file);
        let filename = file.as_path().file_stem().unwrap().to_str().unwrap();
        let _engine = CompilationEngine::compile(filename, tknzr).map_err(|_| {});
    }
}
