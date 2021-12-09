use crate::{Board, Color};
use crate::chess_structs::{Index2D, Vector2D};
use crate::generator2::{Delta, is_square_empty, is_square_enemy, MoveType, push_if_legal};

pub(super) fn generate_pawn(board: &Board, piece_pos: Index2D, color: Color) -> Vec<Delta> {
    let mut moves = Vec::new();

    let delta_y = match color {
        Color::White => {  1 }
        Color::Black => { -1 }
    };

    let pawn_move = |vector: Vector2D| {
        if let Some(landing_pos) = piece_pos + vector {
            if is_square_empty(board, landing_pos) {
                push_if_legal(&mut moves, board, MoveType::Regular, piece_pos, landing_pos);
            }
        } // else the move landed outside of board.
    };

    let pawn_capture = |vector: Vector2D| {
        if let Some(landing_pos) = piece_pos + vector {
            if is_square_enemy(board, landing_pos) {
                let captured_piece = board.get(landing_pos).expect("Should have been an enemy");
                push_if_legal(&mut moves, board, MoveType::Capture(captured_piece), piece_pos, landing_pos);
            }
        } // else the move landed outside of board.
    };

    let one_forward_vect = Vector2D { x: 0, y: delta_y };
    pawn_move(one_forward_vect);

    let two_forward_vect = Vector2D { x: 0, y: 2 * delta_y };
    pawn_move(two_forward_vect);

    let capture_right_vect = Vector2D { x: 1, y: delta_y };
    pawn_capture(capture_right_vect);

    let capture_left_vect = Vector2D { x: -1, y: delta_y };
    pawn_capture(capture_left_vect);

    return moves;
}