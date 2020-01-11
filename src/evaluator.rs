
use crate::chess_structs::{Board, Color, Kind, Piece, Vector2D, Index2D};
use crate::chess_structs::Color::{White, Black};
use crate::chess_structs::Kind::*;

// Attack vector kind mask bits
const KING_VECTOR:   i32 = 1 << 1;
const QUEEN_VECTOR:  i32 = 1 << 2;
const ROOK_VECTOR:   i32 = 1 << 3;
const BISHOP_VECTOR: i32 = 1 << 4;
const KNIGHT_VECTOR: i32 = 1 << 5;
const WHITE_PAWN:    i32 = 1 << 6;
const BLACK_PAWN:    i32 = 1 << 7;

/// Attack vectors, as seen from the kings perspective
const VECTORS: [(Vector2D, i32, i32); 8+8] = [
    (Vector2D {x:  1, y:  1}, 7, KING_VECTOR | QUEEN_VECTOR | BISHOP_VECTOR | BLACK_PAWN),
    (Vector2D {x:  1, y:  0}, 7, KING_VECTOR | QUEEN_VECTOR | ROOK_VECTOR),
    (Vector2D {x:  1, y: -1}, 7, KING_VECTOR | QUEEN_VECTOR | BISHOP_VECTOR | WHITE_PAWN),
    (Vector2D {x:  0, y: -1}, 7, KING_VECTOR | QUEEN_VECTOR | ROOK_VECTOR),
    (Vector2D {x: -1, y: -1}, 7, KING_VECTOR | QUEEN_VECTOR | BISHOP_VECTOR | WHITE_PAWN),
    (Vector2D {x: -1, y:  0}, 7, KING_VECTOR | QUEEN_VECTOR | ROOK_VECTOR),
    (Vector2D {x: -1, y:  1}, 7, KING_VECTOR | QUEEN_VECTOR | BISHOP_VECTOR | BLACK_PAWN),
    (Vector2D {x:  0, y:  1}, 7, KING_VECTOR | QUEEN_VECTOR | ROOK_VECTOR),
    (Vector2D {x:  1, y:  2}, 1, KNIGHT_VECTOR),
    (Vector2D {x:  2, y:  1}, 1, KNIGHT_VECTOR),
    (Vector2D {x:  2, y: -1}, 1, KNIGHT_VECTOR),
    (Vector2D {x:  1, y: -2}, 1, KNIGHT_VECTOR),
    (Vector2D {x: -1, y: -2}, 1, KNIGHT_VECTOR),
    (Vector2D {x: -2, y: -1}, 1, KNIGHT_VECTOR),
    (Vector2D {x: -2, y:  1}, 1, KNIGHT_VECTOR),
    (Vector2D {x: -1, y:  2}, 1, KNIGHT_VECTOR)
];

#[derive(Copy, Clone, Debug)]
pub struct Evaluation {
    score: i32,
    is_white_checked: bool,
    is_black_checked: bool,
    is_castling_blocked: bool
}

pub fn eval(board: &Board) -> Evaluation {
    let score: i32 = board.squares.iter()
        .flat_map(|squares| squares.iter()
            .map(|square| match square {
                None => 0,
                Some(piece) => match piece.color {
                    Color::White => piece.kind.value(),
                    Color::Black => -piece.kind.value()
                }
            })
        )
        .sum();

    let (is_white_checked, is_black_checked) = checks(board);

    Evaluation {
        score,
        is_white_checked,
        is_black_checked,
        is_castling_blocked: false
    }
}

fn checks(board: &Board) -> (bool, bool) {

    let mut is_white_checked = false;
    let mut is_black_checked = false;

    board.squares
        .iter()
        .enumerate()
        .flat_map(|(y, squares)| squares
            .iter()
            .enumerate()
            .filter_map(move |(x, square)| match square {
                Some(piece)=> Some((Index2D::new(x, y), piece)),
                None => None
            })
        )
        .filter(|(pos, piece)| piece.kind == King)
        .map(|(pos, king)| is_check(board, pos, king))
        .for_each(|(color, is_checked)| match color {
            White => is_white_checked = is_checked,
            Black => is_black_checked = is_checked
        });

    (is_white_checked, is_black_checked)
}

fn is_check(board: &Board, pos: Index2D, king: &Piece) -> (Color, bool) {

    let enemy_color = king.color.invert();
    let mut is_check = false;

    'outer: for (vec, reps, kind_mask) in VECTORS.iter() {
        let mut next_square = pos;
        for rep in 0..*reps {
            next_square += vec;
            if next_square.is_out_of_board() {
                break;
            }
            if let Some(piece) = board.squares[next_square.y][next_square.x] {
                if piece.color == enemy_color {
                    // check if this piece can attack along this vector
                    is_check = match piece.kind {
                        Pawn => match piece.color {
                            Black => rep == 0 && kind_mask & BLACK_PAWN > 0,
                            White => rep == 0 && kind_mask & WHITE_PAWN > 0
                        },
                        Bishop => kind_mask & BISHOP_VECTOR > 0,
                        Knight => kind_mask & KNIGHT_VECTOR > 0,
                        Rook => kind_mask & ROOK_VECTOR > 0,
                        King => rep == 0 && kind_mask & KING_VECTOR > 0,
                        Queen => kind_mask & QUEEN_VECTOR > 0,
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


mod tests {
    use crate::chess_structs::{Index2D, Board, Piece};
    use crate::chess_structs::Color::{White, Black};
    use crate::chess_structs::Kind::{Pawn, King, Knight, Rook};
    use crate::evaluator::eval;

    #[test]
    fn test_checks_black_pawn() {
        let attacker_sq = Index2D {x: 3, y: 3};
        let friendly_sq = Index2D {x: 4, y: 3};
        for y in 0..8 {
            for x in 0..8 {
                let new_sq = Index2D::new(x, y);
                if new_sq == friendly_sq {
                    continue;
                }
                let mut board = Board::new(White, None, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind:King, color:White });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind:Pawn, color:Black });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind:Pawn, color:White });
                let evaluation = eval(&board);
                // black pawn attacks downwards + left/right; check if we're in its path
                if y == attacker_sq.y - 1 && (x == attacker_sq.x - 1 || x == attacker_sq.x + 1) {
                    assert!(evaluation.is_white_checked);
                } else {
                    assert!(!evaluation.is_white_checked);
                }
                assert!(!evaluation.is_black_checked);
            }
        }
    }

    #[test]
    fn test_checks_white_pawn() {
        let attacker_sq = Index2D {x: 3, y: 3};
        let friendly_sq = Index2D {x: 4, y: 3};
        for y in 0..8 {
            for x in 0..8 {
                let new_sq = Index2D::new(x, y);
                if new_sq == friendly_sq {
                    continue;
                }
                let mut board = Board::new(Black, None, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind:King, color:Black });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind:Pawn, color:White });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind:Pawn, color:Black });
                let evaluation = eval(&board);
                // white pawn attacks upwards + left/right; check if we're in its path
                if y == attacker_sq.y + 1 && (x == attacker_sq.x - 1 || x == attacker_sq.x + 1) {
                    assert!(evaluation.is_black_checked);
                } else {
                    assert!(!evaluation.is_black_checked);
                }
                assert!(!evaluation.is_white_checked);
            }
        }
    }

    #[test]
    fn test_checks_bishop() {
    }

    #[test]
    fn test_checks_knight() {
        let attacker_sq = Index2D {x: 3, y: 3};
        let friendly_sq = Index2D {x: 4, y: 3};
        let attacked_sqs = [
            Index2D::new(4, 5),
            Index2D::new(5, 4),
            Index2D::new(5, 2),
            Index2D::new(4, 1),
            Index2D::new(2, 1),
            Index2D::new(1, 2),
            Index2D::new(1, 4),
            Index2D::new(2, 5),
        ];
        for y in 0..8 {
            for x in 0..8 {
                let new_sq = Index2D::new(x, y);
                if new_sq == friendly_sq {
                    continue;
                }
                let mut board = Board::new(White, None, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind:King, color:White });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind:Knight, color:Black });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind:Knight, color:White });
                let evaluation = eval(&board);
                if attacked_sqs.iter().any(|attacked_sq| new_sq == *attacked_sq) {
                    assert!(evaluation.is_white_checked);
                } else {
                    assert!(!evaluation.is_white_checked);
                }
                assert!(!evaluation.is_black_checked);
            }
        }
    }

    #[test]
    fn test_checks_rook() {
        let attacker_sq = Index2D {x: 3, y: 3};
        let friendly_sq = Index2D {x: 4, y: 3};
        let attacked_sqs = [
            Index2D::new(3, 4), // UP
            Index2D::new(3, 5),
            Index2D::new(3, 6),
            Index2D::new(3, 7),
            Index2D::new(4, 3), // RIGHT (a friendly piece is blocking the rest of this vector)
            Index2D::new(3, 2), // DOWN
            Index2D::new(3, 1),
            Index2D::new(3, 0),
            Index2D::new(2, 3), // LEFT
            Index2D::new(1, 3),
            Index2D::new(0, 3),
        ];
        for y in 0..8 {
            for x in 0..8 {
                let new_sq = Index2D::new(x, y);
                if new_sq == friendly_sq {
                    continue;
                }
                let mut board = Board::new(White, None, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind:King, color:White });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind:Rook, color:Black });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind:Rook, color:White });
                let evaluation = eval(&board);
                if attacked_sqs.iter().any(|attacked_sq| new_sq == *attacked_sq) {
                    assert!(evaluation.is_white_checked);
                } else {
                    assert!(!evaluation.is_white_checked);
                }
                assert!(!evaluation.is_black_checked);
            }
        }
    }

    #[test]
    fn test_checks_queen() {
    }

    #[test]
    fn test_checks_king() {
    }

    #[test]
    fn test_checks_block() {
        // test that friendly pieces will block checks
    }
}