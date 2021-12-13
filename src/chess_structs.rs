
use std::ops;

pub type CastlingBitField = u32;
pub type ChecksBitField = u8;

pub const WHITE_KINGSIDE: CastlingBitField  = 1 << 0;
pub const WHITE_QUEENSIDE: CastlingBitField = 1 << 1;
pub const BLACK_KINGSIDE: CastlingBitField  = 1 << 2;
pub const BLACK_QUEENSIDE: CastlingBitField = 1 << 3;
pub const CASTLING_WHITE: CastlingBitField = WHITE_KINGSIDE | WHITE_QUEENSIDE;
pub const CASTLING_BLACK: CastlingBitField = BLACK_KINGSIDE | BLACK_QUEENSIDE;

pub const CASTLING_UNAVAILABLE: CastlingBitField = 0;
pub const CASTLING_FULL: CastlingBitField = CASTLING_WHITE| CASTLING_BLACK;

pub const WHITE_IS_CHECKED: ChecksBitField = 1 << 0;
pub const BLACK_IS_CHECKED: ChecksBitField = 1 << 1;

pub const NO_CHECKS: ChecksBitField = 0;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black
}
impl Color {
    pub fn invert(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Index2D {
    pub x: usize,
    pub y: usize,
}
impl Index2D {
    pub fn new(x: usize, y: usize) -> Index2D {
        Index2D {
            x,
            y
        }
    }

    pub fn is_out_of_board(self) -> bool {
        self.x > 7 || self.y > 7
    }
}
// TODO: remove checked add when thoroughly tested
impl ops::AddAssign<&Vector2D> for Index2D {
    fn add_assign(&mut self, rhs: &Vector2D) {
        self.x = ((self.x as i64).checked_add(rhs.x)).unwrap() as usize;
        self.y = ((self.y as i64).checked_add(rhs.y)).unwrap() as usize;
    }
}
// TODO: remove checked add when thoroughly tested
impl ops::Add<Vector2D> for Index2D {
    type Output = Option<Index2D>;

    fn add(self, rhs: Vector2D) -> Self::Output {
        let x = (self.x as i64).checked_add(rhs.x);
        let y = (self.y as i64).checked_add(rhs.y);
        if x.is_some() && y.is_some() {
            let result = Index2D {
                x: x.unwrap() as usize,
                y: y.unwrap() as usize
            };
            if result.is_out_of_board() {
                None
            } else {
                Some(result)
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector2D {
    pub x: i64,
    pub y: i64,
}
impl Vector2D {
    pub fn new(x: i64, y: i64) -> Vector2D {
        Vector2D {
            x,
            y
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub en_passant: Option<Index2D>,
    pub castling_availability: u32,
    pub checks: u8,
}

impl Board {
    pub fn print(&self) {
        for rank in 0..8 {
            let rank= 7 - rank;
            print!(" {} | ", rank+1);
            for file in 0..8 {
                let square = match self.squares[rank][file] {
                    Some(piece) => piece.to_char(),
                    None => '.'
                };
                print!(" {} ", square);
            }
            println!();
        }

        println!("   +  -  -  -  -  -  -  -  -");
        print!("     ");
        for c in "abcdefgh".chars() {
            print!(" {} ", c);
        }
        println!();
    }

    pub fn new(turn: Color, en_passant: Option<Index2D>, castling_availability: CastlingBitField, checks: ChecksBitField) -> Board {
        Board {
            squares: [
                [None; 8], [None; 8], [None; 8], [None; 8], [None; 8], [None; 8], [None; 8], [None; 8]
            ],
            turn,
            en_passant,
            castling_availability,
            checks
        }
    }

    pub fn get(&self, index: Index2D) -> Option<Piece> {
        self.squares[index.y][index.x]
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
}

impl Kind {
    pub fn value(&self) -> i32 {
        match self {
            Kind::Pawn   => 100,
            Kind::Bishop => 300,
            Kind::Knight => 300,
            Kind::Rook   => 500,
            Kind::King   => 9999,
            Kind::Queen  => 900
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color
}


impl Piece {
    pub fn to_char(&self) -> char {
        match self.kind {
            Kind::Pawn => {
                match self.color {
                    Color::White => 'P',
                    Color::Black => 'p',
                }
            }
            Kind::Bishop => {
                match self.color {
                    Color::White => 'B',
                    Color::Black => 'b',
                }
            }
            Kind::King => {
                match self.color {
                    Color::White => 'K',
                    Color::Black => 'k',
                }
            }
            Kind::Knight => {
                match self.color {
                    Color::White => 'N',
                    Color::Black => 'n',
                }
            }
            Kind::Queen => {
                match self.color {
                    Color::White => 'Q',
                    Color::Black => 'q',
                }
            }
            Kind::Rook => {
                match self.color {
                    Color::White => 'R',
                    Color::Black => 'r',
                }
            }
        }
    }
}
