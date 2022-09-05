use crate::snake::game::Direction;
use crate::snake::math::Point;
use crate::snake::pixels::{Color, Rect, Window};

pub struct Board {
    window: Window,
    width: i32,
    height: i32,
}

impl Board {
    pub fn get_size_tile(&self) -> Point {
        Point::new(self.window.width()/self.width, self.window.height()/self.height)
    }
    pub fn new(window: Window, width: i32) -> Self{
        let height = width as f32/window.get_ratio();
        Self{
            height: height as i32,
            width,
            window,
        }
    }
    pub fn handle_events(&mut self){
        self.window.handle_events();
    }
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
    pub fn get_next_direction(&mut self) -> Option<Direction>{
        self.window.get_next_direction()
    }
    pub fn clear(&mut self, color: Color){
        self.window.clear(color);
    }
    pub fn draw_cell(&mut self, x: i32, y: i32, color: Color){
        assert!(x >= 0 && y >= 0);
        assert!(x < self.width && y < self.height);

        let cell_size = self.get_size_tile();
        let pos = Point::new(x, y);

        let rect = Rect{
            center: &(&pos * &cell_size) + &(&cell_size/2),
            half_size: &cell_size/2
        };

        self.window.draw_rect(rect, color);
    }
    pub fn draw_borders(&mut self, color: Color){
        for i in 0..self.width {
            self.draw_cell(i, 0, color.clone());
            self.draw_cell(i, self.height-1, color.clone());
        }
        for i in 1..(self.height-1){
            self.draw_cell(0, i, color.clone());
            self.draw_cell(self.width-1, i, color.clone());
        }
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn draw(&mut self){
        self.window.draw();
    }
}