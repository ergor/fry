use crate::chess_structs::Board;
use crate::generator::IteratorItr;
use crate::chess_structs::Color::White;
use crate::evaluator::{eval};

const MAX_DEPTH: i32 = 10;



pub fn search(board: &Board) -> () {
    let move_iter = board.iter();

    let moves: Vec<Board> = move_iter.flat_map(|iter| iter.map(|a| a)).collect();
    let evals: Vec<(&Board, i32)> = moves.iter().map(|board| (board, minimax(board, MAX_DEPTH, board.turn == White))).collect();
    for (board, eval) in evals {
        board.print();
        println!("Evaluation: {}\n", eval);
    }
}


fn minimax(board: &Board, depth: i32, is_whites_turn: bool) -> i32 {
    if depth == 0 {
        return eval(board);
    }

    if is_whites_turn {
        let max_eval = i32::min_value();
        let move_iter = board.iter();
        let moves: Vec<Board> = move_iter.flat_map(|iter| iter.map(|a| a)).collect();
    }

    return eval(board);
}
