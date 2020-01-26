use std::iter::Iterator;
use crate::chess_structs::{Board, Index2D, Color, Kind, Piece, Vector2D};
use crate::chess_structs::Kind::{Knight, Rook, Bishop, Queen};

// idea: generate most likely board first, specific for black and white

#[macro_export]
macro_rules! board_stream {
    ( $board:expr ) => {
        $board.iter().flatten();
    };
}

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

#[derive(Copy, Clone)]
pub struct IteratorItr<'a> {
    /// The board we're generating moves from.
    board: &'a Board,

    /// The x-index on the board where we are currently looking for a piece
    x: usize,

    /// The y-index on the board where we are currently looking for a piece
    y: usize,
}
impl<'a> IteratorItr<'a> {
    /// Used for when finding the next piece to move.
    fn inc_pos(&mut self) {
        if self.x < 7 {
            self.x += 1;
        }
        else {
            self.x = 0;
            self.y += 1;
        }
    }
}

impl Board {
    pub fn iter(&self) -> IteratorItr {
        IteratorItr {
            board: self,
            x: 0,
            y: 0
        }
    }
}

/// Iterates over every square on the board and tries to find pieces to move.
/// When a piece of correct color is found, returns the Iterator of that piece which
/// will actually generate boards according to how that piece can move.
impl<'a> Iterator for IteratorItr<'a> {
    type Item = Box<dyn Iterator<Item = Board> + 'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inc_pos();
        if self.y > 7 {
            None
        } else {
            match self.board.squares[self.y][self.x] {
                Some(piece) => {
                    if piece.color == self.board.turn  {
                        match piece.kind {
                            Kind::Knight => {
                                let knight_itr = KnightItr::new(self.board, Index2D{x: self.x, y: self.y});
                                Some(Box::new(knight_itr))
                            }
                            Kind::King => {
                                let king_itr = KingItr::new(self.board, Index2D{x: self.x, y: self.y});
                                Some(Box::new(king_itr))
                            }
                            Kind::Pawn => {
                                let pawn_itr = PawnItr::new(self.board, Index2D{x: self.x, y: self.y});
                                Some(Box::new(pawn_itr))
                            }
                            Kind::Rook => {
                                let rook_itr = RookItr::new(self.board, Index2D{x: self.x, y: self.y});
                                Some(Box::new(rook_itr))
                            }
                            Kind::Bishop => {
                                let bishop_itr = BishopItr::new(self.board, Index2D{x: self.x, y: self.y});
                                Some(Box::new(bishop_itr))
                            }
                            Kind::Queen => {
                                let queen_itr = QueenItr::new(self.board, Index2D{x: self.x, y: self.y});
                                Some(Box::new(queen_itr))
                            }
                        }
                    } else {
                        self.next()
                    }
                },
                None => self.next()
            }
        }
    }
}

#[derive(Clone)]
struct GenericItr<'a> {
    initial_board: &'a Board,
    initial_pos: Index2D,
    current_itrn: i32
}


struct RookItr<'a>(GenericItr<'a>);

impl<'a> RookItr<'a> {
    pub fn new(board: &Board, pos: Index2D) -> RookItr {
        RookItr(GenericItr {
            initial_board: board,
            initial_pos: pos,
            current_itrn: 1
        })
    }
}

impl<'a> Iterator for RookItr<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        next_long_move(&mut self.0, true, false)
    }
}
struct BishopItr<'a>(GenericItr<'a>);

impl<'a> BishopItr<'a> {
    pub fn new(board: &Board, pos: Index2D) -> BishopItr {
        BishopItr(GenericItr {
            initial_board: board,
            initial_pos: pos,
            current_itrn: 1
        })
    }
}

impl<'a> Iterator for BishopItr<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        next_long_move(&mut self.0, false, true)
    }
}
struct QueenItr<'a>(GenericItr<'a>);

impl<'a> QueenItr<'a> {
    pub fn new(board: &Board, pos: Index2D) -> QueenItr {
        QueenItr(GenericItr {
            initial_board: board,
            initial_pos: pos,
            current_itrn: 1
        })
    }
}

impl<'a> Iterator for QueenItr<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        next_long_move(&mut self.0, true, true)
    }
}

fn next_long_move(itr: &mut GenericItr, is_up_down:bool, is_diagonal:bool) -> Option<Board> {
    match itr.current_itrn {
        1 ..= 7 => {
            if !is_up_down {
                itr.current_itrn = 29;
                next_long_move(itr, is_up_down, is_diagonal)
            } else {
                let x = itr.current_itrn as i64;
                let y = 0;
                let next_board = next_move(Vector2D::new(x, y), 1, itr);
                if next_board.is_none() {
                    itr.current_itrn = 8;
                    next_long_move(itr, is_up_down, is_diagonal)
                } else {
                    next_board
                }
            }
        }
        8 ..= 14 => {
            let x = -(itr.current_itrn as i64 -7);
            let y = 0;
            let next_board = next_move(Vector2D::new(x, y), 1, itr);
            if next_board.is_none() {
                itr.current_itrn = 15;
                next_long_move(itr, is_up_down, is_diagonal)
            } else {
                next_board
            }
        }
        15 ..= 21 => {
            let x = 0;
            let y = itr.current_itrn as i64 - 14;
            let next_board = next_move(Vector2D::new(x, y), 1, itr);
            if next_board.is_none() {
                itr.current_itrn = 22;
                next_long_move(itr, is_up_down, is_diagonal)
            } else {
                next_board
            }
        }
        22 ..= 28 => {
            let x = 0;
            let y = -(itr.current_itrn as i64 - 21);
            let next_board = next_move(Vector2D::new(x, y), 1, itr);
            if next_board.is_none() {
                itr.current_itrn = 29;
                next_long_move(itr, is_up_down, is_diagonal)
            } else {
                next_board
            }
        }//diagonal
        29 ..= 35 => {
            if !is_diagonal {
              None
            } else {
                let x = itr.current_itrn as i64 - 28;
                let y = itr.current_itrn as i64 - 28;
                let next_board = next_move(Vector2D::new(x, y), 1, itr);
                if next_board.is_none() {
                    itr.current_itrn = 36;
                    next_long_move(itr, is_up_down, is_diagonal)
                } else {
                    next_board
                }
            }
        }
        36 ..= 42 => {
            let x = -(itr.current_itrn as i64 - 35);
            let y = itr.current_itrn as i64 - 35;
            let next_board = next_move(Vector2D::new(x, y), 1, itr);
            if next_board.is_none() {
                itr.current_itrn = 43;
                next_long_move(itr, is_up_down, is_diagonal)
            } else {
                next_board
            }
        }
        43 ..= 49 => {
            let x = itr.current_itrn as i64 - 42;
            let y = -(itr.current_itrn as i64 - 42);
            let next_board = next_move(Vector2D::new(x, y), 1, itr);
            if next_board.is_none() {
                itr.current_itrn = 50;
                next_long_move(itr, is_up_down, is_diagonal)
            } else {
                next_board
            }
        }
        50 ..= 56 => {
            let x = -(itr.current_itrn as i64 - 49);
            let y = -(itr.current_itrn as i64 - 49);
            let next_board = next_move(Vector2D::new(x, y), 1, itr);
            if next_board.is_none() {
                None
            } else {
                next_board
            }
        }
        _ => None
    }
}

struct KingItr<'a>(GenericItr<'a>);

impl<'a> KingItr<'a> {
    pub fn new(board: &Board, pos: Index2D) -> KingItr {
        KingItr(GenericItr {
            initial_board: board,
            initial_pos: pos,
            current_itrn: 1
        })
    }
}

impl<'a> Iterator for KingItr<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        let mut out_of_moves = false;
        let board = match self.0.current_itrn {
            1 => {
                next_move(Vector2D::new(1, 0), 1, &mut self.0)
            }
            2 => {
                next_move(Vector2D::new(1, 1), 1,  &mut self.0)
            }
            3 => {
                next_move(Vector2D::new(1, -1), 1, &mut self.0)
            }
            4 => {
                next_move(Vector2D::new(-1, 0), 1, &mut self.0)
            }
            5 => {
                next_move(Vector2D::new(-1, 1), 1, &mut self.0)
            }
            6 => {
                next_move(Vector2D::new(-1, -1), 1, &mut self.0)
            }
            7 => {
                next_move(Vector2D::new(0, -1), 1, &mut self.0)
            }
            8 => {
                next_move(Vector2D::new(0, 1), 1,  &mut self.0)
            }
            _ => {
                out_of_moves = true;
                None
            }
        };
        if out_of_moves {
            None
        } else {
            match board {
                Some(_) => board,
                None => self.next()
            }
        }
     }
 }

struct KnightItr<'a>(GenericItr<'a>);

impl<'a> KnightItr<'a> {
    pub fn new(board: &Board, pos: Index2D) -> KnightItr {
        KnightItr(GenericItr {
            initial_board: board,
            initial_pos: pos,
            current_itrn: 1
        })
    }
}

impl<'a> Iterator for KnightItr<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        let mut out_of_moves = false;
        let board = match self.0.current_itrn {
            1 => {
                next_move(Vector2D::new(2, 1), 1, &mut self.0)
            }
            2 => {
                next_move(Vector2D::new(2, -1), 1,  &mut self.0)
            }
            3 => {
                next_move(Vector2D::new(-2, 1), 1, &mut self.0)
            }
            4 => {
                next_move(Vector2D::new(-2, 1), 1, &mut self.0)
            }
            5 => {
                next_move(Vector2D::new(1, 2), 1, &mut self.0)
            }
            6 => {
                next_move(Vector2D::new(1, -2), 1, &mut self.0)
            }
            7 => {
                next_move(Vector2D::new(-1, 2), 1, &mut self.0)
            }
            8 => {
                next_move(Vector2D::new(-1, -2), 1,  &mut self.0)
            }
            _ => {
                out_of_moves = true;
                None
            }
        };
        if out_of_moves {
            None
        } else {
            match board {
                Some(_) => board,
                None => self.next()
            }
        }
    }
}

struct PawnItr<'a>(GenericItr<'a>);

impl<'a> PawnItr<'a> {
    pub fn new(board: &Board, pos: Index2D) -> PawnItr {
        PawnItr(GenericItr {
            initial_board: board,
            initial_pos: pos,
            current_itrn: 1
        })
    }
}

impl<'a> Iterator for PawnItr<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        let mut side = 0;
        let mut start_pos_y = 1;
        if self.0.initial_board.turn == Color::Black {
            side = -1;
            start_pos_y = 6;
        }

        let mut out_of_moves = false;
        let board = match self.0.current_itrn {
            1 => {
                if self.0.initial_pos.y == start_pos_y {
                   None
                } else {
                    next_move2(Vector2D::new(0, 2 * side), 1, &mut self.0, is_square_empty)
                }
            }
            2 => {
                next_move2(Vector2D::new(0, 1 * side), 1,  &mut self.0, is_square_empty)
            }
            3 => {
                next_move2(Vector2D::new(-1, 1 * side), 1, &mut self.0, is_square_enemy)
            }
            4 => {
                next_move2(Vector2D::new(1, 1 * side), 1, &mut self.0, is_square_enemy)
            }
            _ => {
                out_of_moves = true;
                None
            }
        };
        if out_of_moves {
            None
        } else {
            match board {
                Some(_) => board,
                None => self.next()
            }
        }
    }
}

pub fn create_new_board(board: &Board, from: Index2D, to: Index2D) -> Board {
    //println!("from x: {}", from.x);
    //println!("from y: {}", from.y);
    //println!("to x: {}", to.x);
    //println!("to y: {}", to.y);
    let mut board = *board;
    board.squares[to.y][to.x] = board.squares[from.y][from.x];
    board.squares[from.y][from.x] = None;
    board.turn = board.turn.invert();

    let (is_white_checked, is_black_checked) = checks(&board);
    board.is_white_checked = is_white_checked;
    board.is_black_checked = is_black_checked;

    board
}

fn next_move (vect: Vector2D, inc: i32, itr: &mut GenericItr) -> Option<Board> {
    next_move2(vect, inc, itr, is_square_empty_or_enemy)
}

fn next_move2 (vect: Vector2D, inc: i32, itr: &mut GenericItr, square_checker: fn(board: &Board, to: Index2D) -> bool) -> Option<Board> {
    let new_pos = itr.initial_pos + vect;

    if let Some(new_pos) = new_pos {
        if new_pos.is_out_of_board() {
            itr.current_itrn += inc;
            None
        }
        else if square_checker(itr.initial_board, new_pos) {
            itr.current_itrn += 1;
            let new_board = create_new_board(itr.initial_board, itr.initial_pos, new_pos);
            match itr.initial_board.turn {
                Color::White => if new_board.is_white_checked { None } else { Some(new_board) },
                Color::Black => if new_board.is_black_checked { None } else { Some(new_board) }
            }
        }
        else {
            itr.current_itrn += 1;
            None
        }
    } else {
        itr.current_itrn += 1;
        None
    }
}

pub fn is_square_empty_or_enemy(board: &Board, to: Index2D) -> bool {
    match board.squares[to.y][to.x] {
        Some(piece) =>  piece.color != board.turn,
        None => true
    }
}

pub fn is_square_empty(board: &Board, to: Index2D) -> bool {
    match board.squares[to.y][to.x] {
        Some(_) =>  false,
        None => true
    }
}

pub fn is_square_enemy(board: &Board, to: Index2D) -> bool {
    match board.squares[to.y][to.x] {
        Some(piece) =>  piece.color != board.turn,
        None => false
    }
}

/// Return which kings are in check. Order: white, black
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
        .filter(|(_, piece)| piece.kind == Kind::King)
        .map(|(pos, king)| is_check(board, pos, king))
        .for_each(|(color, is_checked)| match color {
            Color::White => is_white_checked = is_checked,
            Color::Black => is_black_checked = is_checked
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
                        Kind::Pawn => match piece.color {
                            Color::Black => rep == 0 && kind_mask & BLACK_PAWN > 0,
                            Color::White => rep == 0 && kind_mask & WHITE_PAWN > 0
                        },
                        Kind::Bishop => kind_mask & BISHOP_VECTOR > 0,
                        Kind::Knight => kind_mask & KNIGHT_VECTOR > 0,
                        Kind::Rook => kind_mask & ROOK_VECTOR > 0,
                        Kind::King => rep == 0 && kind_mask & KING_VECTOR > 0,
                        Kind::Queen => kind_mask & QUEEN_VECTOR > 0,
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
    use crate::chess_structs::{Board, Piece, Index2D, Color, Kind};
    use crate::generator::{KingItr, KnightItr, RookItr, BishopItr};
    use crate::generator;

    #[test]
    fn king_test() {
        let board: Board = Board{
            squares: [
                [None; 8], // bottom of board (y = rank -1 = 0)
                [None; 8],
                [None, None, None, None, Some(Piece{kind: Kind::King, color: Color::White}), None, None, None],
                [None, None, None, None, Some(Piece{kind: Kind::Pawn, color: Color::Black}), None, None, None],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8], // top of board (y = rank -1 = 7)
        ],
            turn: Color::White,
            en_passant: None,
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
            is_white_checked: false,
            is_black_checked: false,
        };
        let pos = Index2D {x: 4, y:2};
        let mut king_itr= KingItr::new(&board, pos);
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        // two moves are blocked because it puts the king in check, thus expect 6 positions
        assert!(king_itr.next().is_none());

        let mut new_board:Option<Board> = None;

        let mut move_itr = board.iter();
        if let Some(mut i) = move_itr.next() {
            let itr = i.as_mut();
            new_board = itr.next();
        }

        assert!(new_board.is_some());
    }
    #[test]
    fn knight_test() {
        let board: Board = Board{
            squares: [
                [None; 8], // bottom of board (y = rank -1 = 0)
                [None; 8],
                [None, None, None, None, Some(Piece{kind: Kind::Knight, color: Color::White}), None, None, None],
                [None, None, None, None, Some(Piece{kind: Kind::Pawn, color: Color::Black}), None, None, None],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8], // top of board (y = rank -1 = 7)
            ],
            turn: Color::White,
            en_passant: None,
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
            is_white_checked: false,
            is_black_checked: false,
        };
        let pos = Index2D {x: 4, y:2};
        let mut knight_iter = KnightItr::new(&board, pos);
        assert!(knight_iter.next().is_some());
        assert!(knight_iter.next().is_some());
        assert!(knight_iter.next().is_some());
        assert!(knight_iter.next().is_some());
        assert!(knight_iter.next().is_some());
        assert!(knight_iter.next().is_some());
        assert!(knight_iter.next().is_some());
        assert!(knight_iter.next().is_some());
        // two moves are blocked because it puts the king in check, thus expect 6 positions
        assert!(knight_iter.next().is_none());


        let mut new_board:Option<Board> = None;

        let mut move_itr = board.iter();
        if let Some(mut i) = move_itr.next() {
            let itr = i.as_mut();
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
            assert!(new_board.is_some());
            new_board = itr.next();
        }

        assert!(new_board.is_none());
    }

    #[test]
    fn rook_test() {
        let board: Board = Board{
            squares: [
                [None; 8], // bottom of board (y = rank -1 = 0)
                [None; 8],
                [None, None, None, None, Some(Piece{kind: Kind::Rook, color: Color::Black}), None, None, None],
                [None, None, None, None, Some(Piece{kind: Kind::Pawn, color: Color::Black}), None, None, None],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8], // top of board (y = rank -1 = 7)
            ],
            turn: Color::Black,
            en_passant: None,
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
            is_white_checked: false,
            is_black_checked: false,
        };
        let pos = Index2D {x: 4, y:2};
        let mut rook_iter = RookItr::new(&board, pos);
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        assert!(rook_iter.next().is_some());
        // two moves are blocked because it puts the king in check, thus expect 6 positions
        assert!(rook_iter.next().is_none());
    }
    #[test]
    fn bishop_test() {
        let board: Board = Board{
            squares: [
                [None; 8], // bottom of board (y = rank -1 = 0)
                [None, None, Some(Piece{kind: Kind::Bishop, color: Color::Black}), None, None, None,  None, None],
                [None; 8],
                [None, None, None, None, Some(Piece{kind: Kind::Pawn, color: Color::Black}), None, None, None],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8], // top of board (y = rank -1 = 7)
            ],
            turn: Color::Black,
            en_passant: None,
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
            is_white_checked: false,
            is_black_checked: false,
        };
        let pos = Index2D {x: 2, y:1};
        let mut bishop_iter = BishopItr::new(&board, pos);
        assert!(bishop_iter.next().is_some());
        assert!(bishop_iter.next().is_some());
        assert!(bishop_iter.next().is_some());
        assert!(bishop_iter.next().is_some());
        assert!(bishop_iter.next().is_some());
        // two moves are blocked because it puts the king in check, thus expect 6 positions
        assert!(bishop_iter.next().is_none());
    }

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
                let mut board = Board::new(Color::White, None, false, false, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind: Kind::King, color: Color::White });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind: Kind::Pawn, color: Color::Black });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind: Kind::Pawn, color: Color::White });
                let (is_white_checked, is_black_checked) = generator::checks(&board);
                // black pawn attacks downwards + left/right; check if we're in its path
                if y == attacker_sq.y - 1 && (x == attacker_sq.x - 1 || x == attacker_sq.x + 1) {
                    assert!(is_white_checked);
                } else {
                    assert!(!is_white_checked);
                }
                assert!(!is_black_checked);
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
                let mut board = Board::new( Color::Black, None, false, false, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind: Kind::King, color: Color::Black });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind: Kind::Pawn, color: Color::White });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind: Kind::Pawn, color: Color::Black });
                let (is_white_checked, is_black_checked) = generator::checks(&board);
                // white pawn attacks upwards + left/right; check if we're in its path
                if y == attacker_sq.y + 1 && (x == attacker_sq.x - 1 || x == attacker_sq.x + 1) {
                    assert!(is_black_checked);
                } else {
                    assert!(!is_black_checked);
                }
                assert!(!is_white_checked);
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
                let mut board = Board::new( Color::White, None, false, false, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind: Kind::King, color: Color::White });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind: Kind::Knight, color: Color::Black });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind: Kind::Knight, color: Color::White });
                let (is_white_checked, is_black_checked) = generator::checks(&board);
                if attacked_sqs.iter().any(|attacked_sq| new_sq == *attacked_sq) {
                    assert!(is_white_checked);
                } else {
                    assert!(!is_white_checked);
                }
                assert!(!is_black_checked);
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
                let mut board = Board::new( Color::White, None, false, false, false, false, false, false);
                board.squares[y][x] = Some(Piece { kind: Kind::King, color: Color::White });
                board.squares[attacker_sq.y][attacker_sq.x] = Some(Piece { kind: Kind::Rook, color: Color::Black });
                board.squares[friendly_sq.y][friendly_sq.x] = Some(Piece { kind: Kind::Rook, color: Color::White });
                let (is_white_checked, is_black_checked) = generator::checks(&board);
                if attacked_sqs.iter().any(|attacked_sq| new_sq == *attacked_sq) {
                    assert!(is_white_checked);
                } else {
                    assert!(!is_white_checked);
                }
                assert!(!is_black_checked);
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