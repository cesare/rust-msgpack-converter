extern crate docopt;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

extern crate rmp;
extern crate rmp_serde as rmps;

use std::error::Error as StdError;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use docopt::Docopt;
use serde::Serialize;
use rmps::Serializer;

mod error;
use error::Error;

const USAGE: &'static str = "
Usage:
    json2mp [--output=<filename>] [<json-filename>]

Options:
    -o, --output=<filename>    Specify the filename to write to
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_json_filename: Option<String>,
    flag_output: Option<String>,
}

fn parse_args() -> Args {
    Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit())
}

fn load_json(readable: Box<Read>) -> Result<serde_json::Value, Error> {
    serde_json::from_reader(readable).map_err(|e| Error::new(e.description()))
}

fn dump_as_msgpack(value: serde_json::Value, mut writable: Box<Write>) -> Result<(), Error> {
    let mut serializer = Serializer::new(&mut writable);
    value.serialize(&mut serializer).map_err(|e| Error::new(e.description()))
}

fn open_readable(filename: Option<String>) -> Box<Read> {
    match filename {
        Some(name) => {
            let file = File::open(name).unwrap();
            Box::new(file)
        }
        None => Box::new(std::io::stdin()),
    }
}

fn open_writable(filename: Option<String>) -> Box<Write> {
    match filename {
        Some(name) => {
            let file = File::create(name).unwrap();
            Box::new(file)
        }
        None => Box::new(std::io::stdout()),
    }
}

fn main() {
    let args: Args = parse_args();

    let readable = open_readable(args.arg_json_filename);
    let value = load_json(readable).unwrap();

    let writable = open_writable(args.flag_output);
    dump_as_msgpack(value, writable).unwrap();
}
