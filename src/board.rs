use super::canvas::Canvas;
use super::snake::{Direction, Snake};
use stdweb::js;
use stdweb::unstable::TryInto;

pub trait Drawable {
    fn draw(&mut self, canvas: &Canvas, state: &mut State);
}

pub struct State {
    pub board_width: u32,
    pub board_height: u32,
    pub step_ms: u32,
    pub direction: Direction,
}

impl State {
    pub fn new(step_ms: u32, width: u32, height: u32) -> Self {
        Self {
            board_width: width,
            board_height: height,
            step_ms,
            direction: Direction::Right,
        }
    }
}

pub struct Board {
    pub canvas: Canvas,
    width: u32,
    height: u32,
    drawables: Vec<Box<dyn Drawable>>,
    state: State,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        let canvas = Canvas::new("#canvas", width, height);
        let state = State::new(100, width, height);
        let snake = Snake::new(random_u32(width) as i32, random_u32(height) as i32);
        let drawables: Vec<Box<dyn Drawable>> = vec![Box::new(snake)];
        Self {
            canvas,
            width,
            height,
            drawables,
            state,
        }
    }

    pub fn update(&mut self) {
        self.canvas.clear_all();
        for d in self.drawables.iter_mut() {
            d.draw(&self.canvas, &mut self.state);
        }
    }

    pub fn step_ms(&self) -> u32 {
        self.state.step_ms
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.state.direction = direction;
    }
}

fn random_u32(max: u32) -> u32 {
    let random = js! { return Math.floor(Math.random() * @{max}) }
        .try_into()
        .unwrap();
    random
}
