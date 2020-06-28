use super::board::{Drawable, State};
use super::canvas::Canvas;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Head {
    x: u32,
    y: u32,
}

struct Tail {
    x: u32,
    y: u32,
}

pub struct Snake<'a> {
    head: Head,
    tails: Vec<&'a Tail>,
}

impl<'a> Snake<'a> {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            head: Head { x, y },
            tails: Vec::new(),
        }
    }
}

impl<'a> Drawable for Snake<'a> {
    fn draw(&self, canvas: &Canvas, state: &mut State) {
        canvas.draw(self.head.x, self.head.y, "red");
    }
}
