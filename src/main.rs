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
use fen_rs::parse;
use game_state::map_from_libfen;


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

    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//    let fen_string = "r3k2r/p2ppp2/8/8/8/8/P2PP1PP/R1B1KB1R w KQkq - 0 1";
    let fen_state_result = parse(fen_string);
    match fen_state_result {
        Ok(state) => {
            let game_state = map_from_libfen(Color::Black, state);
            let starting_board = game_state.board_state;

            starting_board.print();

            let game_state = GameState::new(fry_color, starting_board);

            let mut board = starting_board;
            let plies = 0; // half moves played
            loop {
                if board.turn == fry_color {
                    let mut read_buf = String::new();
                    print!("move> ");
                    let player_move = std::io::stdin().read_line(&mut read_buf);
//                    board.turn = board.turn.invert();
                    if let Some(new_board) = minimax::search(&board) {
                        board = new_board;
                        board.print();
                        print!("B");
                    } else {
                        println!("no more legal moves");
                        break;
                    }
                }
                else {
                    let mut read_buf = String::new();
                    print!("move> ");
                    let player_move = std::io::stdin().read_line(&mut read_buf);
//                    board.turn = board.turn.invert();
                    if let Some(new_board) = minimax::search(&board) {
                        board = new_board;
                        board.print();
                        print!("A");
                    } else {
                        println!("no more legal moves");
                        break;
                    }
                }
            }
        }
        Err(..) => {}
    }
}
