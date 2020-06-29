use super::board::{random_u32, Drawable, State};
use super::canvas::Canvas;
use super::collision::Collidable;
use super::snake::Snake;

pub struct Food {
    pub x: i32,
    pub y: i32,
    pub consumed: bool,
}

impl Food {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            consumed: false,
        }
    }
}

impl Collidable for Food {
    fn consumed_by(&mut self, snake: &mut Snake, state: &State) {
        snake.add_tail();
        self.consumed = true;
        let x = random_u32(state.board_width);
        let y = random_u32(state.board_height);
        self.respawn(x as i32, y as i32);
    }

    fn respawn(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.consumed = false;
    }

    fn check_collide(&self, x: i32, y: i32) -> bool {
        if self.x == x && self.y == y {
            true
        } else {
            false
        }
    }
}

impl Drawable for Food {
    fn draw(&mut self, canvas: &Canvas, state: &mut State) {
        canvas.draw(self.x as u32, self.y as u32, "blue");
    }
}
