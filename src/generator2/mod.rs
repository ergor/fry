// in the search tree: instead of storing whole boards, store only the delta (move) from previous node.
// root node must be a whole board, but all sub nodes can be derived from deltas.

mod pawn;
mod bishop;
mod knight;
mod rook;
mod king;
mod queen;
mod check_checker;

use crate::{Board, Color, Kind, Piece};
use crate::chess_structs::{BLACK_KINGSIDE, BLACK_QUEENSIDE, CASTLING_BLACK, CASTLING_WHITE, Index2D, Vector2D, WHITE_KINGSIDE, WHITE_QUEENSIDE};
use crate::Kind::King;

const VECTORS_JUMP: [Vector2D; 8] = [
    Vector2D {x:  2, y:  1},
    Vector2D {x:  2, y: -1},
    Vector2D {x:  1, y:  2},
    Vector2D {x:  1, y: -2},
    Vector2D {x: -1, y:  2},
    Vector2D {x: -1, y: -2},
    Vector2D {x: -2, y:  1},
    Vector2D {x: -2, y: -1},
];

const VECTORS_LINEAR: [Vector2D; 8] = [
    Vector2D {x:  1, y:  1},
    Vector2D {x:  1, y:  0},
    Vector2D {x:  1, y: -1},
    Vector2D {x:  0, y:  1},
    Vector2D {x:  0, y: -1},
    Vector2D {x: -1, y:  1},
    Vector2D {x: -1, y:  0},
    Vector2D {x: -1, y: -1},
];

#[derive(Debug)]
pub enum MoveType {
    /// just a regular move.
    Regular,
    /// the captured piece is stored here, needed for undoing a move.
    Capture(Piece),
    /// this move is an en passant move.
    EnPassant,
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

fn is_square_en_passant(board: &Board, to: Index2D) -> bool {
    match board.en_passant {
        Some(en_passant_pos) => to == en_passant_pos,
        None => false
    }
}

fn is_square_empty_or_enemy(board: &Board, to: Index2D) -> bool {
    match board.get(to) {
        Some(piece) =>  piece.color != board.turn,
        None => true
    }
}

pub fn make_move(the_move: &Delta, board: &mut Board) {
    let moving_piece = board.get(the_move.src)
        .expect("generator: tried to make move from empty square");

    let castling_bits = match moving_piece.color {
        Color::White => CASTLING_WHITE,
        Color::Black => CASTLING_BLACK,
    };
    let is_castling_available = board.castling_availability & castling_bits > 0;

    board.set(the_move.src, Option::None);
    board.set(the_move.dst, Some(moving_piece));

    // deal with weird piece movements
    match the_move.move_type {
        MoveType::EnPassant => {
            let captured_pawn_index = Index2D {
                x: the_move.dst.x,
                y: match moving_piece.color {
                    Color::White => 4,
                    Color::Black => 3,
                }
            };
            board.set(captured_pawn_index, Option::None);
        },
        MoveType::Castle => {
            // if you castle, you lose all castling options.
            board.castling_availability &= !castling_bits;
        },
        MoveType::Regular if moving_piece.kind == Kind::Rook && moving_piece.color == Color::White && is_castling_available => {
            // if you move rook, you lose only one castling option.
            if board.castling_availability & WHITE_KINGSIDE > 0 && the_move.src == KINGSIDE_ROOK_WHITE_INITIAL {

            }
            else if the_move.src == QUEENSIDE_ROOK_WHITE_INITIAL { // is_castling_available && !WHITE_KINGSIDE => QUEENSIDE

            }
        }
        MoveType::Regular if moving_piece.kind == Kind::Pawn => {
            if let Color::White = moving_piece.color {
                if the_move.src.y == 1 && the_move.dst.y == 3 {

                }
            }
            if let Color::Black = moving_piece.color {
                if the_move.src.y == 6 && the_move.dst.y == 4 {

                }
            }
        },
        _ => {},
    }

    // deal with castling state
    if board.castling_availability > 0 {
        if let MoveType::Capture(_) = the_move.move_type {

        }
    }

    board.turn = moving_piece.color.invert();
}

pub fn unmake_move(the_move: &Delta, board: &mut Board) {
    let reverting_piece = board.get(the_move.dst)
        .expect("generator: tried to unmake move from empty square");

    board.set(the_move.src, Some(reverting_piece));
    board.set(the_move.dst, Option::None);

    //...
}

fn push_if_legal(moves: &mut Vec<Delta>, board: &Board, move_type: MoveType, from: Index2D, to: Index2D) {
    let delta = Delta {
        move_type,
        src: from,
        dst: to
    };
    moves.push(delta);
}