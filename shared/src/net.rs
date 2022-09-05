use crate::game::Direction;
use crate::maths::{Color, Point};

pub struct Player{
    name: String,
    pos: Point,
    body: Vec<Point>,
    direction: Direction,
    color: Color
}
pub struct Game {
    window: Point,
    board: Point
}