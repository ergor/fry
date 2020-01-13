use crate::chess_structs::Board;
use crate::chess_structs::Color::White;
use crate::evaluator;
use std::cmp;

const INITIAL_DEPTH: i32 = 5;

static mut NODES_VISITED: i64 = 0;

pub fn search(board: &Board) -> () {
    unsafe { NODES_VISITED = 0; }
    let moves: Vec<Board> = board_stream!(board).collect();
    let evals: Vec<(&Board, i32)> = moves.iter().map(|board| (board, minimax(board, INITIAL_DEPTH, i32::min_value(), i32::max_value(), board.turn == White))).collect();
    for (board, eval) in evals {
        board.print();
        println!("Evaluation: {}\n", eval);
        unsafe { println!("moves computed: {}", NODES_VISITED); }
    }
}

// idea: if minimax returns integer min or max, that means someone was out of moves.
// detect check mate like that?

fn minimax(board: &Board, depth: i32, mut alpha: i32, mut beta: i32, is_whites_turn: bool) -> i32 {

    if depth == 0 {
        unsafe { NODES_VISITED += 1; }
        return evaluator::eval(board);
    }

    if is_whites_turn {
        let mut max_eval = i32::min_value();
        for next_board in board_stream!(board) {
            let eval = minimax(&next_board, depth-1, alpha, beta, false);
            max_eval = cmp::max(eval, max_eval);
            alpha = cmp::max(eval, alpha);
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    }
    else {
        let mut min_eval = i32::max_value();
        for next_board in board_stream!(board) {
            let eval = minimax(&next_board, depth-1, alpha, beta, true);
            min_eval = cmp::min(eval, min_eval);
            beta = cmp::min(eval, beta);
            if beta <= alpha {
                break;
            }
        }
        return min_eval;
    }
}
