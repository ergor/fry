
use crate::chess_structs::{Board, Color, Kind, Piece, Index2D};
use crate::chess_structs::Color::{White, Black};
use crate::chess_structs::Kind::King;

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

    Evaluation {
        score,
        is_white_checked: is_check(board, White),
        is_black_checked: is_check(board, Black),
        is_castling_blocked: false
    }
}

fn is_check(board: &Board, king_color: Color) -> bool {
    let mut opt_king = None;
    let mut opt_pos = None;

    'outer: for y in 0..8 {
        for x in 0..8 {
            if let Some(piece) = board.squares[y][x] {
                if piece.kind == King && piece.color == king_color {
                    opt_king = Some(piece);
                    opt_pos = Some(Index2D::new(x, y));
                    break 'outer;
                }
            }
        }
    }

    if let None = opt_king {
        return false;
    }

    let king = opt_king.unwrap();
    true
}