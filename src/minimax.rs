use crate::chess_structs::Board;
use crate::generator::IteratorItr;
use crate::chess_structs::Color::White;
use crate::evaluator;
use std::cmp;

const INITIAL_DEPTH: i32 = 2;

pub fn search(board: &Board) -> () {
    let move_iter = board.iter();

    let moves: Vec<Board> = board_stream!(board).collect();
    let evals: Vec<(&Board, i32)> = moves.iter().map(|board| (board, minimax(board, INITIAL_DEPTH, board.turn == White))).collect();
    for (board, eval) in evals {
        board.print();
        println!("Evaluation: {}\n", eval);
    }
}

// idea: if minimax returns integer min or max, that means someone was out of moves.
// detect check mate like that?

fn minimax(board: &Board, depth: i32, is_whites_turn: bool) -> i32 {
    //println!("minimax: depth {}", depth);
    if depth == 0 {
        //println!("reached leaf: {}", evaluator::eval(board));
        println!("reached leaf");
        board.print();
        println!("Eval: {}", evaluator::eval(board));
        return evaluator::eval(board);
    }

    if is_whites_turn {
        //println!("maximizing player");
        let mut max_eval = i32::min_value();
        for next_board in board_stream!(board) {
            //println!("maximizing player: new board..");
            let eval = minimax(&next_board, depth-1, false);
            max_eval = cmp::max(eval, max_eval);
        }
        return max_eval;
    }
    else {
        //println!("minimizing player");
        let mut min_eval = i32::max_value();
        for next_board in board_stream!(board) {
            //println!("minimizing player: new board..");
            let eval = minimax(&next_board, depth-1, true);
            min_eval = cmp::min(eval, min_eval)
        }
        return min_eval;
    }
}
