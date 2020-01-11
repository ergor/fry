

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, PartialEq)]
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
}

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub en_passant: Option<Index2D>,
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Board {
    pub fn get_next_turn(&mut self) -> Color {
        if self.turn == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }

    pub fn print(&self) {
        for rank in 0..8 {
            print!(" {} | ", 8 - (rank));
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
}

#[derive(Copy, Clone, PartialEq)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
}

#[derive(Copy, Clone)]
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
