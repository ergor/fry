// in the search tree: instead of storing whole boards, store only the delta (move) from previous node.
// root node must be a whole board, but all sub nodes can be derived from deltas.

mod pawn;
mod bishop;
mod knight;
mod rook;
mod king;
mod queen;

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
        Kind::Pawn   => pawn::generate_pawn(board, piece_pos, piece.color),
        Kind::Bishop => bishop::generate_bishop(board, piece_pos),
        Kind::Knight => knight::generate_knight(board, piece_pos),
        Kind::Rook   => rook::generate_rook(board, piece_pos),
        Kind::King   => king::generate_king(board, piece_pos),
        Kind::Queen  => queen::generate_queen(board, piece_pos),
    }
}

fn is_square_empty(board: &Board, to: Index2D) -> bool {
    match board.get(to) {
        Some(_) => false,
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

fn is_check(board: &Board, king_pos: Index2D, king: &Piece) -> (Color, bool) {

    let enemy_color = king.color.invert();
    let mut is_check = false;

    'outer: for (vec, reps, kind_mask) in THREAT_VECTORS.iter() {
        let mut next_square = king_pos;
        for rep in 0..*reps {
            next_square += vec;
            if next_square.is_out_of_board() {
                break;
            }
            if let Some(piece) = board.get(next_square) {
                if piece.color == enemy_color {
                    // check if this piece can attack along this vector
                    is_check = match piece.kind {
                        Kind::Pawn => match piece.color {
                            Color::Black => kind_mask & THREAT_BLACK_PAWN > 0,
                            Color::White => kind_mask & THREAT_WHITE_PAWN > 0
                        },
                        Kind::Bishop => kind_mask & THREAT_BISHOP > 0,
                        Kind::Knight => kind_mask & THREAT_KNIGHT > 0,
                        Kind::Rook => kind_mask & THREAT_ROOK > 0,
                        Kind::King => kind_mask & THREAT_KING > 0,
                        Kind::Queen => kind_mask & THREAT_QUEEN > 0,
                    };
                    if is_check {
                        break 'outer; // no need to search any more
                        // TODO: is the above true for knights?
                    }
                } else {
                    break; // a friendly piece is blocking this attack vector; on to the next vector!
                    // TODO: is the above true for knights?
                }
            }
        }
    }

    (king.color, is_check)
}

fn push_if_legal(moves: &mut Vec<Delta>, board: &Board, move_type: MoveType, from: Index2D, to: Index2D) {
    let delta = Delta {
        move_type,
        src: from,
        dst: to
    };
    moves.push(delta);
}