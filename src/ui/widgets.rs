use macroquad::prelude::*;

pub trait Widget {
    fn draw(&self);
    fn set_pos(&self, new_pos: Vec2);
    fn get_size(&self) -> Vec2;
}
