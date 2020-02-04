
use std::fs;
use std::io::Read;

use crate::chess_structs::Color;
use clap::{App, Arg, ArgMatches};



const ARG_FILE: &str = "file";
const ARG_COLOR: &str = "color";


pub struct ExpectedActual<'a> {
    pub expected: &'a str,
    pub actual: String,
}

pub enum ArgError<'a> {
    Required(&'a str),
    Invalid(&'a str, ExpectedActual<'a>)
}

pub struct FryArgs {
    pub color: Color,
    pub load_file: Option<String>
}

pub fn parse_args() -> Result<FryArgs, ArgError<'static>> {
    let args = App::new("fry chess engine")
        .version("0.1.0")
        .about("Wait, I'm having one of those things, you know, a headache with pictures.")
        .arg(Arg::with_name(ARG_FILE)
            .short("f")
            .long("file")
            .takes_value(true)
            .help("path to .fen or .pgn file to load as starting point"))
        .arg(Arg::with_name(ARG_COLOR)
            .short("c")
            .long("color")
            .takes_value(true)
            .help("the color fry shall play as (w or b, default b)"))
        .get_matches();

    let load_file = args.value_of(ARG_FILE)
        .map(|val| String::from(val))
        .or(None);

    let color_str = args.value_of(ARG_COLOR).unwrap_or("b");
    let color = match color_str {
        "w" => Ok(Color::White),
        "b" => Ok(Color::Black),
        _ => Err(ArgError::Invalid("Invalid value for color.", ExpectedActual { expected: "'w' or 'b'", actual: String::from(color_str) }))
    }?;

    return Ok(FryArgs {
        color,
        load_file
    });
}
