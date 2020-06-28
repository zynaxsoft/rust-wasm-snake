use super::canvas::Canvas;
use super::snake::Snake;
use stdweb::js;
use stdweb::unstable::TryInto;

pub trait Drawable {
    fn draw(&self, canvas: &Canvas, state: &mut State);
}

pub struct State {
    step_ms: u32,
}

impl State {
    pub fn new(step_ms: u32) -> Self {
        Self { step_ms }
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
        let state = State::new(1000);
        let snake = Snake::new(random_u32(width), random_u32(height));
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
        for d in self.drawables.iter() {
            d.draw(&self.canvas, &mut self.state);
        }
    }

    pub fn step_ms(&self) -> u32 {
        self.state.step_ms
    }
}

fn random_u32(max: u32) -> u32 {
    let random = js! { return Math.floor(Math.random() * @{max}) }
        .try_into()
        .unwrap();
    random
}
