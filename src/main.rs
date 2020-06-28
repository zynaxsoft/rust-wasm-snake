use std::cell::RefCell;
use std::rc::Rc;
use stdweb::js;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    event::{IKeyboardEvent, KeyDownEvent},
    set_timeout, IEventTarget,
};

mod board;
mod canvas;
mod snake;

fn main() {
    stdweb::initialize();

    let board = Rc::new(RefCell::new(board::Board::new(20, 20)));

    let listen_movement = {
        let board = board.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => board.borrow_mut().update(),
                "ArrowRight" => (),
                "ArrowUp" => (),
                "ArrowDown" => (),
                _ => (),
            };
        }
    };
    document().add_event_listener(listen_movement);

    game_loop(board.clone(), 1000);
}

fn game_loop(board: Rc<RefCell<board::Board>>, step_ms: u32) {
    set_timeout(
        move || {
            board.borrow_mut().update();
            game_loop(board, step_ms);
        },
        step_ms,
    )
}
