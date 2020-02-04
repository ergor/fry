use crate::chess_structs;
use crate::chess_structs::{Color, Board, Index2D, Piece};
use crate::libmappings::mappings_fenrs;

pub struct GameState {
    fry_color: Color,
    board_state: Board,
    /// moves since last capture or pawn move
    half_moves: i32,
    moves: Vec<san_rs::Move>,
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

    pub fn map_from_libfen(fry_color: Color, game_state: fen_rs::GameState) -> GameState {

        let active_color = mappings_fenrs::map_color(game_state.active_color);
        let en_passant = match game_state.en_passant {
            Some(pos) => Some(mappings_fenrs::map_position(pos)),
            None => None
        };

        let mut board_state = Board::new(
            active_color,
            en_passant,
            mappings_fenrs::map_castling_availability(game_state.castling_availability),
            chess_structs::NO_CHECKS);

        for rank in game_state.pieces.iter() {
            for piece in rank {
                if let Some(piece) = piece {
                    let pos = mappings_fenrs::map_position(piece.position);
                    board_state.squares[pos.y][pos.x] = Some(mappings_fenrs::map_piece(*piece));
                }
            }
        }

        let half_moves = game_state.half_move_clock;

        GameState {
            fry_color,
            board_state,
            half_moves,
            moves: vec![]
        }
    }
}