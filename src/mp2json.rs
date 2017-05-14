extern crate docopt;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;

extern crate rmp;
extern crate rmp_serde as rmps;

use std::error::Error as StdError;
use std::io::Read;
use std::io::Write;

use docopt::Docopt;
use serde::Deserialize;
use rmps::Deserializer;

mod error;
mod io;
use error::Error;
use io::{open_readable, open_writable};

const USAGE: &'static str = "
Usage:
    mp2json [--output=<filename>] [--pretty] [<msgpack-filename>]

Options:
    -o, --output=<filename>    Specify the filename to write to
    --pretty                   Show JSON in pretty format
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_msgpack_filename: Option<String>,
    flag_output: Option<String>,
    flag_pretty: bool,
}

fn parse_args() -> Args {
    Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit())
}

fn load_msgpack(readable: Box<Read>) -> Result<serde_json::Value, Error> {
    let mut deserializer = Deserializer::new(readable);
    Deserialize::deserialize(&mut deserializer).map_err(|e| Error::new(e.description()))
}

fn dump_as_json(value: &serde_json::Value,
                writable: Box<Write>,
                pretty: bool)
                -> Result<(), Error> {
    if pretty {
        serde_json::to_writer_pretty(writable, value).map_err(|e| Error::new(e.description()))
    } else {
        serde_json::to_writer(writable, value).map_err(|e| Error::new(e.description()))
    }
}

fn main() {
    let args: Args = parse_args();

    let readable = open_readable(args.arg_msgpack_filename);
    let value = load_msgpack(readable).unwrap();

    let writable = open_writable(args.flag_output);
    dump_as_json(&value, writable, args.flag_pretty).unwrap();
}
