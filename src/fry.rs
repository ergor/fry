
extern crate termcolor;

mod gameset;
use gameset::{PieceData, Board, Position};

use std::io;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let starter_board = gameset::generate_starting_board();
    print_board(&starter_board);
}

fn print_board_files() {
    print!("   ");
    for c in (b'a'..=b'h').map(char::from) {
        print!(" {} ", c);
    }
    println!("");
}

fn print_board(board: &Board) {

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let white_sq = (Some(Color::Black), Some(Color::White));
    let black_sq = (Some(Color::White), Some(Color::Black));

    let mut print_sq = | colors: (Option<Color>, Option<Color>),
                         c: char |
                       -> io::Result<()> {
        stdout.set_color(ColorSpec::new().set_fg(colors.0))?;
        stdout.set_color(ColorSpec::new().set_bg(colors.1))?;
        write!(stdout, " {} ", c)?;
        stdout.reset()
    };

    print_board_files();

    for y in 0..8 {
        for x in -1..9 {
            // print ranks
            if x == -1 || x == 8 {
                print!(" {} ", (7-y) + 1);
                continue;
            }
            // print the squares
            print_sq(
                if (x+y) & 1 == 0 {white_sq} else {black_sq},
                match board.piece_at(&gameset::Position::new(x, y)) {
                    Some(piece) => piece.piece().character(),
                    None => ' '
                }
            ).unwrap();
        }
        println!("");
    }

    print_board_files();
}

