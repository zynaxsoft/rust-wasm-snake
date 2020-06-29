use super::board::State;
use super::snake::Snake;

pub trait Collidable {
    fn check_collide(&self, x: i32, y: i32) -> bool;
    fn consumed_by(&mut self, snake: &mut Snake, state: &State);
    fn respawn(&mut self, x: i32, y: i32) {}
}
