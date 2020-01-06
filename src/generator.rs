use std::iter::Iterator;
use crate::chess_structs::{Board, Index2D, Color, Kind};
use san_rs::Piece;
use std::convert::TryInto;

// idea: generate most likely board first, specific for black and white
// TODO fix grid index directions x y


#[derive(Copy, Clone)]
pub(crate) struct MoveItr {
    board: Board,
    x: usize,
    y: usize,
}
impl MoveItr {
    pub fn new(board:Board) -> MoveItr {
        MoveItr {
            board,
            x: 0,
            y: 0
        }
    }
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

pub fn next_move(board_itr: &mut dyn Iterator<Item = Board>, y_vec:i32, x_vec:i32, inc: i32) -> Option<Board> {
    let opt_y = add(board_itr.pos.y, y_vec);
    let opt_x = add(board_itr.pos.x, x_vec);
    let is_legal = false;
    if let Some(new_y) = opt_y {
        if let Some(new_x) = opt_x {
            if is_out_of_board(board_itr.pos, new_y, new_x) {
                board_itr.nr += inc;
                board_itr.next()
            }
            else if is_legal_move(board_itr.curr, board_itr.pos, Index2D{x: new_x, y:new_y}) {
                board_itr.nr += 1;
                Some(create_new_board(board_itr.curr, board_itr.pos, Index2D{x: new_x, y:new_y}))
            }
            else {
                board_itr.nr += 1;
                board_itr.next()
            }
        } else {
            board_itr.nr += 1;
            board_itr.next()
        }
    } else {
        board_itr.nr += 1;
        board_itr.next()
    }
}

pub fn create_new_board(board: Board, from: Index2D, to: Index2D) -> Board {
    println!("from x: {}", from.x);
    println!("from y: {}", from.y);
    println!("to x: {}", to.x);
    println!("to y: {}", to.y);
    let mut board = board;
    board.squares[to.y][to.x] = board.squares[from.y][from.x];
    board.squares[from.y][from.x] = None;
    board.turn = board.get_next_turn();

    let board = board;
    board
}

pub fn is_legal_move(board: Board, _from: Index2D, to: Index2D) -> bool {
    match board.squares[to.y][to.x] {
        Some(piece) =>  piece.color != board.turn,
        None => true
    }
}

pub fn is_out_of_board(old_pos:Index2D, y:usize, x:usize) -> bool {
    if  x > 7 || y > 7 {
        true
    } else {
        false
    }
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

mod tests {
    use crate::chess_structs::{Board, Color, Piece, Index2D};
    use crate::chess_structs::Kind::{King, Pawn};
    use crate::generator::{KingItr, MoveItr};

    #[test]
    fn king_test() {
        let board: Board = Board{
            squares: [
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None, None, None, None, Some(Piece{kind:Pawn, color:Color::Black}), None, None, None],
                [None, None, None, None, Some(Piece{kind:King, color:Color::White}), None, None, None],
                [None; 8],
                [None; 8],
        ],
            turn: Color::White,
            en_passant: None,
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false
        };
        let pos = Index2D {x: 4, y:5};
        let mut king_itr= KingItr::new(board, pos);
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_some());
        assert!(king_itr.next().is_none());

        let mut new_board:Option<Board> = None;

        let mut move_itr = MoveItr::new(board);
        if let Some(mut i) = move_itr.next() {
            let mut itr = i.as_mut();
            new_board = itr.next();
        }

        assert!(new_board.is_some());
    }
}