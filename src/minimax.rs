use crate::chess_structs::{Board, Color};
use crate::evaluator;
use std::cmp;

const INITIAL_DEPTH: i32 = 5;

static mut NODES_VISITED: i64 = 0;

struct Evaluation<'a> {
    board: &'a Board,
    eval: i32
}

impl<'a> Evaluation<'a> {
    fn is_bested_by(&self, other: &Evaluation, who_played: Color) -> bool {
        match who_played{
            Color::White => other.eval > self.eval,
            Color::Black => other.eval < self.eval
        }
    }
}

pub fn search(initial_board: &Board) -> Option<Board> {
    unsafe { NODES_VISITED = 0; }
    let moves: Vec<Board> = board_stream!(initial_board).collect();
    let evals: Vec<Evaluation> = moves.iter().map(|board| Evaluation { board, eval: minimax(board, INITIAL_DEPTH, i32::min_value(), i32::max_value(), board.turn == Color::White) }).collect();

    unsafe { println!("moves computed: {}", NODES_VISITED); }

    let mut best_move: Option<Evaluation> = None;

    for evaluation in evals {
        let evaluation_ref = &evaluation;
        println!("eval: {}", evaluation_ref.eval);
        if best_move.is_none() {
            println!("new best for {:?}: {}", initial_board.turn, evaluation_ref.eval);
            best_move = Some(evaluation);
        }
        else if let Some(current_best) = &best_move {
            if current_best.is_bested_by(evaluation_ref, initial_board.turn) {
                println!("new best for {:?}: {}", initial_board.turn, evaluation_ref.eval);
                best_move = Some(evaluation);
            }
        }
    }

    if best_move.is_none() {
        return None;
    }

    return Some(best_move.unwrap().board.to_owned());
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
