use std::path::PathBuf;

pub struct Compiler { 
    as_text: bool
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            as_text: false
        }
    }

    pub fn write(path: PathBuf) {
        println!("{} bytes written", 0);
    }
}