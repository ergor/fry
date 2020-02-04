
pub mod mappings_fenrs {
    use crate::chess_structs;
    use crate::chess_structs::{Color, Index2D, Piece, Kind, CastlingBitField};

    pub fn map_color(color: fen_rs::Color) -> Color {
        match color {
            fen_rs::Color::White => Color::White,
            fen_rs::Color::Black => Color::Black
        }
    }

    pub fn map_position(position: fen_rs::Position) -> Index2D {
        Index2D::new(position.0, position.1)
    }

    pub fn map_piece_kind(kind: fen_rs::Kind) -> Kind {
        match kind {
            fen_rs::Kind::Pawn => Kind::Pawn,
            fen_rs::Kind::Rook => Kind::Rook,
            fen_rs::Kind::Knight => Kind::Knight,
            fen_rs::Kind::Bishop => Kind::Bishop,
            fen_rs::Kind::Queen => Kind::Queen,
            fen_rs::Kind::King => Kind::King
        }
    }

    pub fn map_piece(piece: fen_rs::Piece) -> Piece {
        Piece {
            kind: map_piece_kind(piece.kind),
            color: map_color(piece.color)
        }
    }

    pub fn map_castling_availability(input: i32) -> CastlingBitField {
        let mut value: CastlingBitField = chess_structs::CASTLING_UNAVAILABLE;

        if input & fen_rs::WHITE_KINGSIDE > 0 {
            value |= chess_structs::WHITE_KINGSIDE;
        }
        if input & fen_rs::WHITE_QUEENSIDE > 0 {
            value |= chess_structs::WHITE_QUEENSIDE;
        }
        if input & fen_rs::BLACK_KINGSIDE > 0 {
            value |= chess_structs::BLACK_KINGSIDE;
        }
        if input & fen_rs::BLACK_QUEENSIDE > 0 {
            value |= chess_structs::BLACK_QUEENSIDE;
        }

        return value;
    }
}

pub mod mappings_sanrs {

}