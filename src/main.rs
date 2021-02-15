pub mod errors;

use std::env;
use std::fs;
use std::io;
use std::fmt;
use std::io::Read;
use std::error::Error;
use std::process::exit;

enum Input {
    Text(String),
    Binary(Vec<u8>),
}

enum InputOption {
    Text(String),
    Binary(String),
    Stdin,
}

impl InputOption {
    fn filename(&self) -> Option<&str> {
        match self {
            InputOption::Text(f) => Some(f),
            InputOption::Binary(f) => Some(f),
            InputOption::Stdin => None,
        }
    }

    fn read(&self) -> io::Result<Input> {
        match self {
            InputOption::Text(f) => Ok(Input::Binary(fs::read(f)?)),
            InputOption::Binary(f) => Ok(Input::Text(fs::read_to_string(f)?)),
            InputOption::Stdin => {
                let mut stdin = vec![];
                io::stdin().read_to_end(&mut stdin)?;
                if stdin.starts_with(&[0x00, 0x61, 0x73, 0x6d]) {
                    Ok(Input::Binary(stdin))
                } else {
                    match String::from_utf8(stdin) {
                        Ok(s) => Ok(Input::Text(s)),
                        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
                    }
                }
            }
        }
    }
}

struct Options {
    file: InputOption,
    help: bool,
}

fn arg_parse() -> Result<Options, String> {
    let mut file = InputOption::Stdin;
    let mut help = false;

    for arg in env::args().skip(1) {
        if arg == "--help" || arg == "-h" {
            help = true;
            break;
        }

        if let Some(f) = file.filename() {
            return Err(format!(
                "Only one file can be specified for now. But '{}' and '{}' are specified. See --help",
                f, arg
            ));
        }

        if arg.ends_with(".wasm") {
            file = InputOption::Binary(arg);
            continue;
        }
        if arg.ends_with(".wat") {
            file = InputOption::Text(arg);
            continue;
        }

        return Err(format!("File '{}' does not '.wasm/.wat' format.", arg));
    }
    Ok(Options {
        file,
        help,
    })
}

fn help() -> ! {
    println!(
        "\
wasrus: A toy webassembly interpreter written in Rust.

USAGE:
    wasrus [OPTIONS] <file>

OPTIONS:
    --help | -h  : Show this help");
    exit(0);
}


fn unwrap<T, E: fmt::Display>(result: Result<T, E>) -> T {
    match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}

pub fn instantiate<B: AsRef<[u8]>>(
    buf: B,
) -> Result<(), Box<dyn Error>> {
    let mut magic_num = [0; 4];
    let mut reader = buf.as_ref();

    reader.read_exact(&mut magic_num)?;

    let magic_num = String::from_utf8(magic_num.to_vec())?;
    if magic_num != "\0asm" {
        return Err(errors::WasrusError::InvalidWasmFileError)?;
    }
    Ok(())
}

fn main() {
    match arg_parse() {
        Ok(arg) => {
            if arg.help {
                help();
            }
            match unwrap(arg.file.read()) {
                Input::Text(_) => println!("Input is .wat file"),
                Input::Binary(_) => println!("Input is .wasm file"),
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
    //instantiate(buf);
}
