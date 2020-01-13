mod chess_structs;
#[macro_use]
mod generator;
mod evaluator;
mod minimax;

//use san_rs;
use crate::chess_structs::{Board, Piece, Kind, Color};
use crate::generator::IteratorItr;
use crate::evaluator::eval;
use crate::minimax::search;

fn main() {
    let board1: Board = Board {
        squares: [
            [None; 8], // bottom of board (y = rank -1 = 0)
            [None; 8],
            [None, None, None, None, Some(Piece{kind: Kind::King, color: Color::White}), None, None, None],
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

    board1.print();
    search(&board1);

    //let mut move_itr:MoveItr = MoveItr::new(board1);
    //while let Some(mut p_itr) = move_itr.next() {
    //    while let Some(new_board) = p_itr.next() {
    //        new_board.print();
    //        let score = eval(&new_board);
    //        println!("Static evaluation:\n{:?}", score);
    //    }
    //}
}


