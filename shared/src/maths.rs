use std::ops::{Mul, Add, Div, Sub};

#[derive(Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }
}
impl<'a> Mul<&'a Point> for &'a Point {
    type Output = Point;

    fn mul(self, point: Self) -> Self::Output {
        Point{
            x: self.x*point.x,
            y: self.y*point.y
        }
    }
}
impl Mul<i32> for &Point{
    type Output = Point;
    fn mul(self, val: i32) -> Self::Output {
        Point{
            x: self.x*val,
            y: self.y*val
        }
    }
}
impl<'a> Add<&'a Point> for &'a Point{
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x+rhs.x,
            y: self.y+rhs.y
        }
    }
}
impl<'a> Add<i32> for &'a Point{
    type Output = Point;

    fn add(self, rhs: i32) -> Self::Output {
        Point{
            x: self.x+rhs,
            y: self.y+rhs
        }
    }
}
impl<'a> Sub<&'a Point> for &'a Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl<'a> Sub<i32> for &'a Point {
    type Output = Point;

    fn sub(self, rhs: i32) -> Self::Output {
        Point{
            x: self.x-rhs,
            y: self.y-rhs
        }
    }
}
impl<'a> Div<&'a Point> for &'a Point {
    type Output = Point;

    fn div(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x/rhs.x,
            y: self.y/rhs.y
        }
    }
}
impl<'a> Div<i32> for &'a Point{
    type Output = Point;

    fn div(self, rhs: i32) -> Self::Output {
        Point{
            x: self.x/rhs,
            y: self.y/rhs
        }
    }
}

#[derive(Clone)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8
}
impl Color{
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}