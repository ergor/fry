mod chess_structs;
#[macro_use]
mod generator;
mod evaluator;
mod minimax;
mod game_state;

//use san_rs;
use crate::chess_structs::{Board, Piece, Kind, Color};
use crate::game_state::GameState;
use clap::{App, Arg, ArgMatches};
use std::process;
use std::fs;
use std::io::Read;

const ERROR_ARG_INVALID: i32 = 1;

const ARG_FILE: &str = "file";
const ARG_COLOR: &str = "color";

struct FryArgs {
    color: Color,
    load_file: Option<fs::File>
}

fn main() {

    let FryArgs {color: fry_color, load_file } = match parse_args() {
        Ok(args) => args,
        Err((code, msg)) => {
            eprintln!("{}", msg);
            process::exit(code);
        }
    };

    let starting_board = Board {
        squares: [
            [None; 8], // bottom of board (y = rank -1 = 0)
            [None; 8],
            [None, None, None, None, Some(Piece{kind: Kind::King, color: Color::White}), None, None, Some(Piece{kind: Kind::Knight, color: Color::Black})],
            [None, Some(Piece{kind: Kind::King, color: Color::Black}), Some(Piece{kind: Kind::Knight, color: Color::Black}), None, Some(Piece{kind: Kind::Pawn, color: Color::Black}), None, None, None],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8], // top of board (y = rank -1 = 7)
        ],
        turn: Color::White,
        en_passant: None,
        white_kingside: false,
        white_queenside: false,
        black_kingside: false,
        black_queenside: false,
        is_white_checked: false,
        is_black_checked: false,
    };
    starting_board.print();

    let game_state = GameState::new(fry_color, starting_board);

    let mut board = starting_board;
    let plies = 0; // half moves played
    loop {
        if board.turn == fry_color {
            if let Some(new_board) = minimax::search(&board) {
                board = new_board;
                board.print();
            } else {
                println!("no more legal moves");
                break;
            }
        }
        else {
            let mut read_buf = String::new();
            print!("move> ");
            let player_move = std::io::stdin().read_line(&mut read_buf);
            board.turn = board.turn.invert();
        }
    }
}

fn parse_args() -> Result<FryArgs, (i32, &'static str)> {
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

    let color = args.value_of(ARG_COLOR).unwrap_or("b");
    let color = match color {
        "w" => Some(Color::White),
        "b" => Some(Color::Black),
        _ => None
    };

    if let None = color {
        return Err((ERROR_ARG_INVALID, "color must be either w or b (for white and black respectively)"));
    }

    return Ok(FryArgs {
        color: color.unwrap(),
        load_file: None
    });
}

