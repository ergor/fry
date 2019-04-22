
mod piece_defs;

use std::vec::Vec;

pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn cmp(&self, pos2: &Position) -> bool {
        self.x == pos2.x && self.y == pos2.y
    }

    pub fn add_vect(&self, vect: (i32, i32)) -> Position {
        let (dx, dy) = vect;
        Position::new(self.x + dx, self.y + dy)
    }
}

pub enum Color {
    WHITE,
    BLACK
}

pub struct Piece {
    color: Color,               // white or black
    position: Position,         // (0,0) top left; (7,7) bottom right
    symbol: char,               // text representation of the piece
    value: i32,                 // the relative value of the piece
    generator: fn(&Piece, &Board) -> Vec<Board>
}

impl Piece {
    pub fn character(&self) -> char {
        match self.color {
            Color::BLACK => self.symbol.to_ascii_lowercase(),
            Color::WHITE => self.symbol.to_ascii_uppercase(),
        }
    }

    pub fn new(color: Color,
               symbol: char,
               value: i32,
               position: Position,
               generator: fn(&Piece, &Board) -> Vec<Board>
            ) -> Piece {
        let value = match &color {
            Color::WHITE => value,
            Color::BLACK => -value,
        };

        Piece {
            color,
            position,
            symbol,
            value,
            generator
        }
    }
}

pub struct Board {
    sum: i32,               // board evaluation
    checks: (u32, u32),     // checks against white,black king
    pub pieces: Vec<Piece>, // the pieces on this board
}

impl Board {
    pub fn new(pieces: Vec<Piece>) -> Board {
        Board {
            sum: 0,
            checks: (0, 0),
            pieces
        }
    }

    pub fn piece_at(&self, pos: &Position) -> Option<&Piece> {
        self.pieces.iter()
            .find(|&p| p.position.cmp(pos))
    }

    pub fn within_bounds(pos: &Position) -> bool {
        pos.x >= 0 && pos.x < 8 && pos.y >= 0 && pos.y < 8
    }
}


pub fn generate_starting_board() -> Board {
    let mut starter_board = Board::new(Vec::new());

    // white pieces
    for x in 0..8 {
        starter_board.pieces.push(piece_defs::new(piece_defs::pawn::def(), Color::WHITE, Position::new(x, 6)));
        //starter_board.pieces.push(Box::new(Pawn::new(Color::WHITE, Position::new(x, 6))));
    }

    starter_board.pieces.push(piece_defs::new(piece_defs::rook::def(), Color::WHITE, Position::new(0, 7)));
    //starter_board.pieces.push(Box::new(Rook::new(Color::WHITE, Position::new(0, 7))));
    //push(PieceClass::KNIGHT, Position::new(1, 7));
    //push(PieceClass::BISHOP, Position::new(2, 7));
    //push(PieceClass::QUEEN,  Position::new(3, 7));
    //push(PieceClass::KING,   Position::new(4, 7));
    //push(PieceClass::BISHOP, Position::new(5, 7));
    //push(PieceClass::KNIGHT, Position::new(6, 7));
    //push(PieceClass::ROOK,   Position::new(7, 7));

    // black pieces
    for x in 0..8 {
        starter_board.pieces.push(piece_defs::new(piece_defs::pawn::def(), Color::BLACK, Position::new(x, 1)));
    }

    //push(PieceClass::ROOK,   Position::new(0, 0));
    //push(PieceClass::KNIGHT, Position::new(1, 0));
    //push(PieceClass::BISHOP, Position::new(2, 0));
    //push(PieceClass::QUEEN,  Position::new(3, 0));
    //push(PieceClass::KING,   Position::new(4, 0));
    //push(PieceClass::BISHOP, Position::new(5, 0));
    //push(PieceClass::KNIGHT, Position::new(6, 0));
    //push(PieceClass::ROOK,   Position::new(7, 0));

    starter_board
}

