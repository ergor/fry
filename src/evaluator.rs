
use crate::chess_structs::{Board, Color};


pub fn eval(board: &Board) -> i32 {
    let score: i32 = board.squares.iter()
        .flat_map(|squares| squares.iter()
            .map(|square| match square {
                None => 0,
                Some(piece) => match piece.color {
                    Color::White => piece.kind.value(),
                    Color::Black => -piece.kind.value()
                }
            })
        )
        .sum();

    score
}


mod tests {

}