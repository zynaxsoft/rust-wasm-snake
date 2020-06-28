use super::board::{Drawable, State};
use super::canvas::Canvas;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    pub fn opposite(&self, other: Self) -> bool {
        match self {
            Self::Up => match other {
                Self::Down => true,
                _ => false,
            },
            Self::Down => match other {
                Self::Up => true,
                _ => false,
            },
            Self::Left => match other {
                Self::Right => true,
                _ => false,
            },
            Self::Right => match other {
                Self::Left => true,
                _ => false,
            },
        }
    }
}

trait Step {
    fn step(&mut self, offset: (i32, i32), state: &State) {
        let (x, y) = self.get_xy();
        let x = x + offset.0;
        let y = y + offset.1;
        self.pass_or_warp(x, y, state);
    }

    fn pass_or_warp(&mut self, x: i32, y: i32, state: &State) {
        let (mut next_x, mut next_y) = (x, y);
        if x < 0 {
            next_x = state.board_width as i32 - 1;
        } else if x > state.board_width as i32 - 1 {
            next_x = 0;
        }
        if y < 0 {
            next_y = state.board_height as i32 - 1;
        } else if y > state.board_height as i32 - 1 {
            next_y = 0;
        }
        self.set_xy(next_x, next_y);
    }

    fn get_xy(&self) -> (i32, i32);
    fn set_xy(&mut self, x: i32, y: i32);
}

struct Head {
    pub x: i32,
    pub y: i32,
}

impl Drawable for Head {
    fn draw(&mut self, canvas: &Canvas, state: &mut State) {
        canvas.draw(self.x as u32, self.y as u32, "red");
    }
}

impl Step for Head {
    fn get_xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn set_xy(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

struct Tail {
    pub x: i32,
    pub y: i32,
}

impl Drawable for Tail {
    fn draw(&mut self, canvas: &Canvas, state: &mut State) {
        canvas.draw(self.x as u32, self.y as u32, "green");
    }
}

impl Step for Tail {
    fn get_xy(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn set_xy(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Snake {
    head: Head,
    tails: Vec<Tail>,
    direction: Direction,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let tails = vec![Tail { x: x - 1, y }, Tail { x: x - 2, y }];
        Self {
            head: Head { x, y },
            tails,
            direction: Direction::Right,
        }
    }

    fn step(&mut self, direction: Direction, state: &mut State) {
        self.direction = direction;
        let offset = self.direction.to_offset();
        let (mut prev_x, mut prev_y) = (self.head.x, self.head.y);
        self.head.step(offset, state);
        for tail in self.tails.iter_mut() {
            let (tmp_x, tmp_y) = (tail.x, tail.y);
            tail.set_xy(prev_x, prev_y);
            prev_x = tmp_x;
            prev_y = tmp_y;
        }
    }
}

impl Drawable for Snake {
    fn draw(&mut self, canvas: &Canvas, state: &mut State) {
        self.step(state.direction, state);
        self.head.draw(canvas, state);
        for tail in self.tails.iter_mut() {
            tail.draw(canvas, state);
        }
    }
}
