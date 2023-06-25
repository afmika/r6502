use std::path::PathBuf;

use r6502::compiler::Compiler;
use clap::Parser;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
enum Mode {
    /// Print compiled hex values
    Hex,
    /// Print parse result of the program
    Parse
}

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "6502 assembly compiler", long_about = None)]
struct Args {
    /// File path
    file: String,
    /// Output path
    output: Option<String>,
    /// Output mode
    #[clap(subcommand)]
    mode: Option<Mode>
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let input = PathBuf::from(args.file);
    let output = match args.output {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("./a.bin"),
    }; 
    
    let mut compiler = Compiler::new(None);
    compiler.init(input)?;

    if let Some(mode) = args.mode {
        match mode {
            Mode::Hex => {
                let hex_string = compiler.to_hex_string()?;
                print!("{}", hex_string);
            },
            Mode::Parse => {
                print!("{}", compiler.get_parse_string());
            }
        }
    } else {
        compiler.run(&output)?;
        println!("Binary generated at {}", output.display());
    }

    Ok(())
}
