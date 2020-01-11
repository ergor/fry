mod chess_structs;
mod generator;
mod evaluator;

//use san_rs;
use crate::chess_structs::{Board, Piece};
use crate::chess_structs::Kind::{Pawn, King};
use crate::chess_structs::Color::{White, Black};
use crate::generator::MoveItr;
use crate::evaluator::eval;

fn main() {
    let board1: Board = Board{
        squares: [
            [None; 8], // bottom of board (y = rank -1 = 0)
            [None; 8],
            [None, None, None, None, Some(Piece{kind:King, color:White}), None, None, None],
            [None, None, None, None, Some(Piece{kind:Pawn, color:Black}), None, None, None],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8], // top of board (y = rank -1 = 7)
        ],
        turn: White,
        en_passant: None,
        white_kingside: false,
        white_queenside: false,
        black_kingside: false,
        black_queenside: false
    };

    board1.print();

    let mut move_itr:MoveItr = MoveItr::new(board1);
    while let Some(mut p_itr) = move_itr.next() {
        while let Some(new_board) = p_itr.next() {
            new_board.print();
            let score = eval(&new_board);
            println!("Static evaluation:\n{:?}", score);
        }
    }
}


