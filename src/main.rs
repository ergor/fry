mod chess_structs;
#[macro_use]
mod generator;
mod evaluator;
mod minimax;
mod game_state;
mod libmappings;
mod args;

use std::process;
use args::FryArgs;
use crate::chess_structs::{Board, Piece, Kind, Color};
use crate::game_state::GameState;
use crate::args::ArgError;


const ERROR_ARG: i32 = 1;

enum ExitCodes {
    InvalidArgument,
    IOError
}

impl ExitCodes {
    fn code(self) -> i32 {
        match self {
            ExitCodes::InvalidArgument => 2,
            ExitCodes::IOError => 1,
        }
    }
}

fn main() {

    let FryArgs {color: fry_color, load_file } = match args::parse_args() {
        Ok(args) => args,
        Err(error) => {
            match error {
                ArgError::Required(msg) => {
                    eprintln!("{}", msg);
                },
                ArgError::Invalid(msg, values) => {
                    eprintln!("{}\n(expected {}; got {})", msg, values.expected, values.actual);
                },
            }

            process::exit(ExitCodes::InvalidArgument.code());
        }
    };

    let starting_board = Board {
        squares: [
            [None; 8], // bottom of board (y = rank -1 = 0)
            [None; 8],
            [None, None, None, None, Some(Piece{kind: Kind::King, color: Color::White}), None, None, Some(Piece{kind: Kind::Pawn, color: Color::White})],
            [None, Some(Piece{kind: Kind::King, color: Color::Black}), Some(Piece{kind: Kind::Rook, color: Color::Black}), None, Some(Piece{kind: Kind::Pawn, color: Color::White}), None, None, None],
            [None; 8],
            [None; 8],
            [None, None, Some(Piece{kind: Kind::Queen, color: Color::White}), None, None, None, None, None],
            [None; 8], // top of board (y = rank -1 = 7)
        ],
        turn: Color::White,
        en_passant: None,
        castling_availability: chess_structs::CASTLING_FULL,
        checks: chess_structs::NO_CHECKS
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
