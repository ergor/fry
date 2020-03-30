
/// # engine.rs
/// Control center for the main thread (user I/O) to interact with engine components
/// running in other threads

use crate::minimax;
use crate::chess_structs::Board;

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

pub type EngineSender = Sender<Command>;
pub type EngineReceiver = Receiver<Option<Board>>;

pub enum Command {
    Execute(Board),
    Stop
}

pub struct ControlCh {
    pub tx: Sender<Command>,
    pub rx: Receiver<Command>,
}

pub struct BoardCh {
    pub tx: Sender<Option<Board>>,
    pub rx: Receiver<Option<Board>>,
}

pub struct Channels {
    pub control_ch: ControlCh,
    pub board_ch: BoardCh
}

pub fn init_engine() -> Channels {
    let (control_tx, control_rx) = mpsc::channel();
    let (data_tx, data_rx) = mpsc::channel();

    Channels {
        control_ch: ControlCh {
            tx: control_tx,
            rx: control_rx
        },
        board_ch: BoardCh {
            tx: data_tx,
            rx: data_rx
        }
    }
}

pub fn spawn(on_executed: fn()) {

    thread::spawn(move || {
        loop {
            let cmd = control_rx.recv().unwrap(); // TODO: error handling
            match cmd {
                Command::Stop => break,
                Command::Execute(board) => {
                    let best_board = minimax::search(&board);
                    data_tx.send(best_board).unwrap();
                    on_executed();
                }
            }
        }
    });


}