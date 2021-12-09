// in the search tree: instead of storing whole boards, store only the delta (move) from previous node.
// root node must be a whole board, but all sub nodes can be derived from deltas.

use crate::{Board, Color, Kind, Piece};
use crate::chess_structs::{Index2D, Vector2D};

// Threat vector kind mask bits
const THREAT_KING      : i32 = 1 << 0;
const THREAT_QUEEN     : i32 = 1 << 1;
const THREAT_ROOK      : i32 = 1 << 2;
const THREAT_BISHOP    : i32 = 1 << 3;
const THREAT_KNIGHT    : i32 = 1 << 4;
const THREAT_WHITE_PAWN: i32 = 1 << 5;
const THREAT_BLACK_PAWN: i32 = 1 << 6;

/// All possible attack vectors from enemies, as seen from the king's perspective.
const THREAT_VECTORS: [(Vector2D, i32, i32); 8+8+8] = [
    (Vector2D {x:  1, y:  1}, 1, THREAT_KING | THREAT_WHITE_PAWN),
    (Vector2D {x:  1, y:  0}, 1, THREAT_KING                    ),
    (Vector2D {x:  1, y: -1}, 1, THREAT_KING | THREAT_BLACK_PAWN),
    (Vector2D {x:  0, y:  1}, 1, THREAT_KING                    ),
    (Vector2D {x:  0, y: -1}, 1, THREAT_KING                    ),
    (Vector2D {x: -1, y:  1}, 1, THREAT_KING | THREAT_WHITE_PAWN),
    (Vector2D {x: -1, y:  0}, 1, THREAT_KING                    ),
    (Vector2D {x: -1, y: -1}, 1, THREAT_KING | THREAT_BLACK_PAWN),

    (Vector2D {x:  1, y:  1}, 7, THREAT_QUEEN | THREAT_BISHOP),
    (Vector2D {x:  1, y:  0}, 7, THREAT_QUEEN | THREAT_ROOK  ),
    (Vector2D {x:  1, y: -1}, 7, THREAT_QUEEN | THREAT_BISHOP),
    (Vector2D {x:  0, y:  1}, 7, THREAT_QUEEN | THREAT_ROOK  ),
    (Vector2D {x:  0, y: -1}, 7, THREAT_QUEEN | THREAT_ROOK  ),
    (Vector2D {x: -1, y:  1}, 7, THREAT_QUEEN | THREAT_BISHOP),
    (Vector2D {x: -1, y:  0}, 7, THREAT_QUEEN | THREAT_ROOK  ),
    (Vector2D {x: -1, y: -1}, 7, THREAT_QUEEN | THREAT_BISHOP),

    (Vector2D {x:  1, y:  2}, 1, THREAT_KNIGHT),
    (Vector2D {x:  2, y:  1}, 1, THREAT_KNIGHT),
    (Vector2D {x:  2, y: -1}, 1, THREAT_KNIGHT),
    (Vector2D {x:  1, y: -2}, 1, THREAT_KNIGHT),
    (Vector2D {x: -1, y: -2}, 1, THREAT_KNIGHT),
    (Vector2D {x: -2, y: -1}, 1, THREAT_KNIGHT),
    (Vector2D {x: -2, y:  1}, 1, THREAT_KNIGHT),
    (Vector2D {x: -1, y:  2}, 1, THREAT_KNIGHT)
];

#[derive(Debug)]
pub enum MoveType {
    /// just a regular move.
    Regular,
    /// the captured piece is stored here, needed for undoing a move.
    Capture(Piece),
    /// long or short castle is derivable from src and dst of the king.
    Castle,
}

#[derive(Debug)]
pub struct Delta {
    move_type: MoveType,
    src: Index2D,
    dst: Index2D,
}

pub fn generate(board: &Board) -> Vec<Delta> {
    let mut all_moves: Vec<Delta> = Vec::new();
    for y in 0..7 {
        for x in 0..7 {
            let position = Index2D {x, y};
            if let Some(piece) = board.get(position) {
                let moves = generate_at(board, position, &piece);
                all_moves.extend(moves);
            }
        }
    }
    return all_moves;
}

fn generate_at(board: &Board, piece_pos: Index2D, piece: &Piece) -> Vec<Delta> {
    match piece.kind {
        Kind::Pawn   => generate_pawn(board, piece_pos, piece.color),
        Kind::Bishop => generate_bishop(board, piece_pos),
        Kind::Knight => generate_knight(board, piece_pos),
        Kind::Rook   => generate_rook(board, piece_pos),
        Kind::King   => generate_king(board, piece_pos),
        Kind::Queen  => generate_queen(board, piece_pos),
    }
}

fn is_square_empty(board: &Board, to: Index2D) -> bool {
    match board.get(to) {
        Some(_) =>  false,
        None => true
    }
}

fn is_square_enemy(board: &Board, to: Index2D) -> bool {
    match board.get(to) {
        Some(piece) =>  piece.color != board.turn,
        None => false
    }
}

fn is_square_empty_or_enemy(board: &Board, to: Index2D) -> bool {
    match board.get(to) {
        Some(piece) =>  piece.color != board.turn,
        None => true
    }
}

fn push_if_legal(moves: &mut Vec<Delta>, board: &Board, move_type: MoveType, from: Index2D, to: Index2D) {
    let delta = Delta {
        move_type,
        src: from,
        dst: to
    };
    moves.push(delta);
}

fn generate_pawn(board: &Board, piece_pos: Index2D, color: Color) -> Vec<Delta> {
    let mut moves = Vec::new();

    let delta_y = match color {
        Color::White => {  1 }
        Color::Black => { -1 }
    };

    let pawn_move: fn(Vector2D) = |vector: Vector2D| {
        if let Some(landing_pos) = piece_pos + vector {
            if is_square_empty(board, landing_pos) {
                push_if_legal(&mut moves, board, MoveType::Regular, piece_pos, landing_pos);
            }
        } // else the move landed outside of board.
    };

    let pawn_capture: fn(Vector2D) = |vector: Vector2D| {
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

fn generate_bishop(board: &Board, piece_pos: Index2D) -> Vec<Delta> {
    Vec::new()
}

fn generate_knight(board: &Board, piece_pos: Index2D) -> Vec<Delta> {
    Vec::new()
}

fn generate_rook(board: &Board, piece_pos: Index2D) -> Vec<Delta> {
    Vec::new()
}

fn generate_king(board: &Board, piece_pos: Index2D) -> Vec<Delta> {
    Vec::new()
}

fn generate_queen(board: &Board, piece_pos: Index2D) -> Vec<Delta> {
    Vec::new()
}
