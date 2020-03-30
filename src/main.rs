mod chess_structs;
#[macro_use]
mod generator;
mod evaluator;
mod minimax;
mod game_state;
mod libmappings;
mod args;
mod uci;
mod engine;

use std::process;
use args::FryArgs;
use crate::chess_structs::{Board, Piece, Kind, Color};
use crate::game_state::GameState;
use crate::args::ArgError;
use std::sync::mpsc::Receiver;

enum ExitCodes {
    InvalidArgument,
    IOError,
    Error,
}

impl ExitCodes {
    fn code(self) -> i32 {
        match self {
            ExitCodes::IOError => 1,
            ExitCodes::InvalidArgument => 2,
            ExitCodes::Error => 3,
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
    let fen_state_result = fen_rs::parse(fen_string);

    if let Err(e) = fen_state_result {
        fen_rs::print_error(e);
        process::exit(ExitCodes::Error.code());
    }

    let game_state = game_state::map_from_libfen(Color::Black, fen_state_result.unwrap());
    let starting_board = game_state.board_state;

    starting_board.print();

    let mut board = starting_board;
    let plies = 0; // half moves played

    let (control_tx, next_board_rx) = engine::init_engine();

    loop {

        if board.turn == fry_color {
            let mut read_buf = String::new();
            print!("move> ");
            let read_result = std::io::stdin().read_line(&mut read_buf);

            control_tx.send(engine::Command::Execute(board)).unwrap(); // TODO: error handling


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


fn on_engine_result(board_rx: engine::EngineReceiver) {
    let result = board_rx.recv();
}