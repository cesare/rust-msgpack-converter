use std::fs::File;
use std::io::{Read, Write};

pub fn open_readable(filename: Option<String>) -> Box<Read> {
    match filename {
        Some(name) => {
            let file = File::open(name).unwrap();
            Box::new(file)
        }
        None => Box::new(::std::io::stdin()),
    }
}

pub fn open_writable(filename: Option<String>) -> Box<Write> {
    match filename {
        Some(name) => {
            let file = File::create(name).unwrap();
            Box::new(file)
        }
        None => Box::new(::std::io::stdout()),
    }
}
