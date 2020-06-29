use super::canvas::Canvas;
use super::collision::Collidable;
use super::consumable::Food;
use super::snake::{Direction, Snake};
use std::cell::RefCell;
use std::rc::Rc;
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
    pub collision_list: Vec<Rc<RefCell<dyn Collidable>>>,
    pub ded: bool,
}

impl State {
    pub fn new(step_ms: u32, width: u32, height: u32) -> Self {
        Self {
            board_width: width,
            board_height: height,
            step_ms,
            direction: Direction::Right,
            collision_list: Vec::new(),
            ded: false,
        }
    }

    pub fn add_collidable(&mut self, collidable: Rc<RefCell<dyn Collidable>>) {
        self.collision_list.push(collidable);
    }
}

pub struct Board {
    pub canvas: Canvas,
    drawables: Vec<Rc<RefCell<dyn Drawable>>>,
    state: State,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        let canvas = Canvas::new("#canvas", width, height);
        let mut state = State::new(100, width, height);
        let snake = Snake::new(random_u32(width) as i32, random_u32(height) as i32);
        let snake = Rc::new(RefCell::new(snake));
        let food = Food::new(random_u32(width) as i32, random_u32(height) as i32);
        let food = Rc::new(RefCell::new(food));
        let drawables: Vec<Rc<RefCell<dyn Drawable>>> = vec![snake.clone(), food.clone()];
        state.add_collidable(food.clone());
        Self {
            canvas,
            drawables,
            state,
        }
    }

    pub fn update(&mut self) {
        self.canvas.clear_all();
        for d in self.drawables.iter_mut() {
            d.borrow_mut().draw(&self.canvas, &mut self.state);
        }
    }

    pub fn step_ms(&self) -> u32 {
        self.state.step_ms
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if !self.state.direction.opposite(direction) {
            self.state.direction = direction;
        }
    }

    pub fn is_game_ded(&self) -> bool {
        self.state.ded
    }
}

pub fn random_u32(max: u32) -> u32 {
    let random = js! { return Math.floor(Math.random() * @{max}) }
        .try_into()
        .unwrap();
    random
}
