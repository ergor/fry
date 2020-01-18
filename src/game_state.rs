use crate::chess_structs::{Color, Board};
use san_rs::Move;

pub struct GameState {
    fry_color: Color,
    board_state: Board,
    /// moves since last capture or pawn move
    half_moves: i32,
    moves: Vec<Move>,
}

impl GameState {
    pub fn new(fry_color: Color, board_state: Board) -> GameState {
        GameState {
            fry_color,
            board_state,
            half_moves: 0,
            moves: Vec::new()
        }
    }
}