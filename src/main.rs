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
mod collision;
mod consumable;
mod snake;

fn main() {
    stdweb::initialize();

    let board = Rc::new(RefCell::new(board::Board::new(20, 20)));

    let listen_movement = {
        let board = board.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => board.borrow_mut().set_direction(snake::Direction::Left),
                "ArrowRight" => board.borrow_mut().set_direction(snake::Direction::Right),
                "ArrowUp" => board.borrow_mut().set_direction(snake::Direction::Up),
                "ArrowDown" => board.borrow_mut().set_direction(snake::Direction::Down),
                _ => (),
            };
        }
    };
    document().add_event_listener(listen_movement);

    board.borrow_mut().update();
    game_loop(board.clone());
}

fn game_loop(board: Rc<RefCell<board::Board>>) {
    let step_ms = board.borrow().step_ms();
    set_timeout(
        move || {
            board.borrow_mut().update();
            if !board.borrow().is_game_ded() {
                game_loop(board);
            } else {
                return;
            }
        },
        step_ms,
    )
}
