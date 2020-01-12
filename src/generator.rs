use std::iter::Iterator;
use crate::chess_structs::{Board, Index2D, Color, Kind, Piece, Vector2D};

// idea: generate most likely board first, specific for black and white

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
pub(crate) struct MoveItr {
    /// The board we're generating moves from.
    board: Board,

    /// The x-index on the board where we are currently looking for a piece
    x: usize,

    /// The y-index on the board where we are currently looking for a piece
    y: usize,
}
impl MoveItr {
    pub fn new(board: Board) -> MoveItr {
        MoveItr {
            board,
            x: 0,
            y: 0
        }
    }

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

/// Iterates over every square on the board and tries to find pieces to move.
/// When a piece of correct color is found, returns the Iterator of that piece which
/// will actually generate boards according to how that piece can move.
impl Iterator for MoveItr {
    type Item = Box<dyn Iterator<Item = Board>>;

    fn next(&mut self) -> Option<Box<dyn Iterator<Item=Board>>> {
        self.inc_pos();
        if self.y > 7 {
            None
        } else {
            match self.board.squares[self.y][self.x] {
                Some(piece) => {
                    match piece.color {
                        Color::White => {
                            match piece.kind {
                                Kind::King => {
                                    let king_itr = KingItr::new(self.board, Index2D{x: self.x, y: self.y});
                                    Some(Box::new(king_itr))
                                }
                                _ => self.next()
                            }
                        },
                        Color::Black => self.next()
                    }
                },
                None => self.next()
            }
        }
    }
}

#[derive(Clone)]
struct KingItr {
    curr: Board,
    pos: Index2D,
    nr: i32
}

impl KingItr {
    pub fn new(board: Board, pos: Index2D) -> KingItr {
        KingItr {
            curr: board,
            pos,
            nr: 1
        }
    }

    fn next_move(&mut self, y_vec:i32, x_vec:i32, inc: i32) -> Option<Board> {
        let opt_y = add(self.pos.y, y_vec);
        let opt_x = add(self.pos.x, x_vec);
        let is_legal = false;
        if let Some(new_y) = opt_y {
            if let Some(new_x) = opt_x {
                let new_pos = Index2D::new(new_x, new_y);
                if new_pos.is_out_of_board() {
                    self.nr += inc;
                    self.next()
                }
                else if is_square_empty_or_enemy(self.curr, self.pos, Index2D{x: new_x, y:new_y}) {
                    self.nr += 1;
                    let new_board = create_new_board(self.curr, self.pos, new_pos);
                    match self.curr.turn {
                        Color::White => if new_board.is_white_checked { self.next() } else { Some(new_board) },
                        Color::Black => if new_board.is_black_checked { self.next() } else { Some(new_board) }
                    }
                }
                else {
                    self.nr += 1;
                    self.next()
                }
            } else {
                self.nr += 1;
                self.next()
            }
        } else {
            self.nr += 1;
            self.next()
        }
    }
}

impl Iterator for KingItr {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        let pos = self.pos;
        match  self.nr{
            1 => {
                self.next_move(0, 1, 3)
            }
            2 => {
                self.next_move(1, 1, 1)
            }
            3 => {
                self.next_move(-1, 1, 1)
            }
            4 => {
                self.next_move(0, -1, 3)
            }
            5 => {
                self.next_move(1, -1, 1)
            }
            6 => {
                self.next_move(-1, -1, 1)
            }
            7 => {
                self.next_move(-1, 0, 1)
            }
            8 => {
                self.next_move(1, 0, 1)
            }
            _ => None
        }
     }
 }


pub fn create_new_board(board: Board, from: Index2D, to: Index2D) -> Board {
    //println!("from x: {}", from.x);
    //println!("from y: {}", from.y);
    //println!("to x: {}", to.x);
    //println!("to y: {}", to.y);
    let mut board = board;
    board.squares[to.y][to.x] = board.squares[from.y][from.x];
    board.squares[from.y][from.x] = None;
    board.turn = board.turn.invert();

    let (is_white_checked, is_black_checked) = checks(&board);
    board.is_white_checked = is_white_checked;
    board.is_black_checked = is_black_checked;

    let board = board;
    board
}

pub fn is_square_empty_or_enemy(board: Board, _from: Index2D, to: Index2D) -> bool {
    match board.squares[to.y][to.x] {
        Some(piece) =>  piece.color != board.turn,
        None => true
    }
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
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
        .filter(|(pos, piece)| piece.kind == Kind::King)
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
    use crate::generator::{KingItr, MoveItr, checks};

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
        let mut king_itr= KingItr::new(board, pos);
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        // two moves are blocked because it puts the king in check, thus expect 6 positions
        assert!(king_itr.next().is_none());

        let mut new_board:Option<Board> = None;

        let mut move_itr = MoveItr::new(board);
        if let Some(mut i) = move_itr.next() {
            let mut itr = i.as_mut();
            new_board = itr.next();
        }

        assert!(new_board.is_some());
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
                let (is_white_checked, is_black_checked) = checks(&board);
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
                let (is_white_checked, is_black_checked) = checks(&board);
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
                let (is_white_checked, is_black_checked) = checks(&board);
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
                let (is_white_checked, is_black_checked) = checks(&board);
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