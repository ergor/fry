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
const THREAT_VECTORS: [(Vector2D, i32, i32); 8+8] = [
    (Vector2D {x:  1, y:  1}, 7, THREAT_KING | THREAT_QUEEN | THREAT_BISHOP | THREAT_WHITE_PAWN),
    (Vector2D {x:  1, y:  0}, 7, THREAT_KING | THREAT_QUEEN | THREAT_ROOK),
    (Vector2D {x:  1, y: -1}, 7, THREAT_KING | THREAT_QUEEN | THREAT_BISHOP | THREAT_BLACK_PAWN),
    (Vector2D {x:  0, y: -1}, 7, THREAT_KING | THREAT_QUEEN | THREAT_ROOK),
    (Vector2D {x: -1, y: -1}, 7, THREAT_KING | THREAT_QUEEN | THREAT_BISHOP | THREAT_BLACK_PAWN),
    (Vector2D {x: -1, y:  0}, 7, THREAT_KING | THREAT_QUEEN | THREAT_ROOK),
    (Vector2D {x: -1, y:  1}, 7, THREAT_KING | THREAT_QUEEN | THREAT_BISHOP | THREAT_WHITE_PAWN),
    (Vector2D {x:  0, y:  1}, 7, THREAT_KING | THREAT_QUEEN | THREAT_ROOK),
    (Vector2D {x:  1, y:  2}, 1, THREAT_KNIGHT),
    (Vector2D {x:  2, y:  1}, 1, THREAT_KNIGHT),
    (Vector2D {x:  2, y: -1}, 1, THREAT_KNIGHT),
    (Vector2D {x:  1, y: -2}, 1, THREAT_KNIGHT),
    (Vector2D {x: -1, y: -2}, 1, THREAT_KNIGHT),
    (Vector2D {x: -2, y: -1}, 1, THREAT_KNIGHT),
    (Vector2D {x: -2, y:  1}, 1, THREAT_KNIGHT),
    (Vector2D {x: -1, y:  2}, 1, THREAT_KNIGHT),
];

fn is_check_linear(board: &Board, king_pos: Index2D, king: &Piece) {
    for i in 0..8 {

    }
}

fn is_check(board: &Board, pos: Index2D, king: &Piece) -> (Color, bool) {

    let enemy_color = king.color.invert();
    let mut is_check = false;

    'outer: for (vec, reps, kind_mask) in THREAT_VECTORS.iter() {
        let mut next_square = pos;
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
                            Color::Black => rep == 0 && kind_mask & THREAT_BLACK_PAWN > 0,
                            Color::White => rep == 0 && kind_mask & THREAT_WHITE_PAWN > 0
                        },
                        Kind::Bishop => kind_mask & THREAT_BISHOP > 0,
                        Kind::Knight => kind_mask & THREAT_KNIGHT > 0,
                        Kind::Rook => kind_mask & THREAT_ROOK > 0,
                        Kind::King => rep == 0 && kind_mask & THREAT_KING > 0,
                        Kind::Queen => kind_mask & THREAT_QUEEN > 0,
                    };
                    if is_check {
                        break 'outer; // no need to search any more
                    }
                } else {
                    break; // a friendly piece is blocking this attack vector; on to the next vector!
                }
            }
        }
    }

    (king.color, is_check)
}