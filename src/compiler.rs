use std::path::PathBuf;

pub struct Compiler { }

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write(path: PathBuf) {
        println!("{} bytes written to {:?}", 0, path);
    }
}