use crate::texture::Texture;
use crate::types::{Rect, Vec2i,I32};
use crate::animation::Animation;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct Sprite {
    image: Rc<Texture>,
    animation: Rc<Animation>, // Maybe better to use a type that can't have a negative origin
    // Or use =animation:Animation= instead of a frame field
    pub position: Vec2i,
    pub direction: Direction,
    pub elapsed_time: usize
}

impl Sprite {
    pub fn new(image: &Rc<Texture>, animation: &Rc<Animation>, position: Vec2i, direction: Direction, elapsed_time: usize) -> Self {
        Self {
            image: Rc::clone(image),
            animation: Rc::clone(animation),
            position,
            direction,
            elapsed_time
        }
    }
    pub fn tick(&mut self){
        self.elapsed_time +=1;
    }
}

/// Returns the row of the spritesheet corresponding to directions
pub fn sheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}
pub trait DrawSpriteExt {
    fn draw_sprite(&mut self, s: &Sprite);
}

use crate::screen::Screen;
impl<'fb> DrawSpriteExt for Screen<'fb> {
    fn draw_sprite(&mut self, s: &Sprite) {
        // This works because we're only using a public method of Screen here,
        // and the private fields of sprite are visible inside this module
        self.bitblt(&s.image, s.animation.get_frame(s.elapsed_time), s.position);
    }
}