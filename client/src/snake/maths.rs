use std::ops::{Add, Div, Mul, Sub};
use snake_shared::maths::Point;

pub struct Rect{
    pub center: Point,
    pub half_size: Point
}


impl From<Rect> for sdl2::rect::Rect {
    fn from(e: Rect) -> Self {
        sdl2::rect::Rect::new(
            e.center.x-e.half_size.x,
            e.center.y-e.half_size.y,
            (e.half_size.x*2) as u32,
            (e.half_size.y*2) as u32,
        )
    }
}