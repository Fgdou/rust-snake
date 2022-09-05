use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use snake_shared::game::Direction;
use snake_shared::maths::Color;
use crate::snake::maths::Rect;

pub fn colorToSdl(c: Color) -> sdl2::pixels::Color {
    sdl2::pixels::Color::RGB(c.r, c.g, c.b)
}

pub struct Window {
    height: i32,
    width: i32,
    canvas: WindowCanvas,
    should_close: bool,
    event: sdl2::EventPump,
    next_direction: Vec<Direction>
}
impl Window {
    pub fn get_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    pub fn new(height: i32, width: i32, name: String) -> Self {
        let sdl_handle = sdl2::init().unwrap();
        let video_handle = sdl_handle.video().unwrap();

        let window = video_handle.window(name.as_str(), width as u32, height as u32)
            .position_centered().build().unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event = sdl_handle.event_pump().unwrap();

        Self {
            height, width, canvas, should_close: false, event,
            next_direction: Vec::new()
        }
    }

    pub fn handle_events(&mut self){
        use Direction::*;

        for e in self.event.poll_iter() {
            match e {
                Event::Quit{..} => self.should_close = true,
                Event::KeyDown {keycode: Some(k), .. } => {
                    if self.next_direction.len() <= 3 {
                        match k {
                            Keycode::Escape => self.should_close = true,
                            Keycode::Left | Keycode::A => self.next_direction.push(LEFT),
                            Keycode::Up | Keycode::W => self.next_direction.push(UP),
                            Keycode::Right | Keycode::D => self.next_direction.push(RIGHT),
                            Keycode::Down | Keycode::S => self.next_direction.push(DOWN),
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
    }
    pub fn get_next_direction(&mut self) -> Option<Direction> {
        if self.next_direction.is_empty() {
            None
        }else {
            Some(self.next_direction.remove(0))
        }
    }

    pub fn clear(&mut self, color: Color){
        self.canvas.set_draw_color(colorToSdl(color));
        self.canvas.clear();
    }
    pub fn draw_rect(&mut self, rect: Rect, color: Color){
        self.canvas.set_draw_color(colorToSdl(color));
        self.canvas.fill_rect( sdl2::rect::Rect::from(rect)).unwrap();
    }

    pub fn draw(&mut self){
        self.canvas.present();
    }

    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn should_close(&self) -> bool {
        self.should_close
    }
}