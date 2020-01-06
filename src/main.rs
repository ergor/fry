mod chess_structs;
mod generator;

//use san_rs;
use crate::chess_structs::{Board, Piece, Color};
use crate::chess_structs::Kind::{Pawn, King};
use crate::generator::MoveItr;

fn main() {
    let board1: Board = Board{
        squares: [
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None, Some(Piece{kind:Pawn, color:Color::White}), None, None, None, None, None, None],
            [ Some(Piece{kind:King, color:Color::White}),None, None, None, None, None, None, None],
        ],
        turn: Color::White,
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
        }
    }
}


