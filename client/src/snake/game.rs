use snake_shared::game::{Direction};
use crate::snake::board::Board;
use snake_shared::game::Direction::UP;
use snake_shared::maths::{Color, Point};
use crate::snake::pixels::Window;

pub struct Game{
    apple: Option<Point>,
    player: Player,
    board: Board,
    running: bool,
    players: Vec<Player>
}
pub struct Player {
    pub name: String,
    pub direction: Direction,
    pub alive: bool,
    pub body: Vec<Option<Point>>
}
impl Player{
    pub fn new(name: String) -> Player {
        Player{
            body: Vec::new(),
            alive: true,
            name,
            direction: UP
        }
    }
    pub fn new_local(name: String, pos: Point) -> Player {
        let body = vec![
            None,
            None,
            Some(pos),
        ];
        Player{
            body,
            direction: UP,
            alive: true,
            name
        }
    }
    fn forward(&mut self, direction: Option<Direction>){
        match direction {
            None => {}
            Some(d) => {
                if !self.direction.opposite(&d) {
                    self.direction = d;
                }
            }
        }


        let head = self.get_head();
        let new_head = match self.direction {
            Direction::UP => head + &Point::new(0, -1),
            Direction::LEFT => head + &Point::new(-1, 0),
            Direction::DOWN => head + &Point::new(0, 1),
            Direction::RIGHT => head + &Point::new(1, 0)
        };

        self.body.remove(0);
        self.body.push(Some(new_head));
    }
    pub fn get_size(&self) -> i32 {
        self.body.len() as i32
    }
    pub fn get_head(&self) -> &Point {
        for i in self.body.iter().rev() {
            match i {
                Some(p) => return p,
                None => {}
            }
        }
        panic!();
    }
    fn organise_player(&mut self){
        let vec = &self.body;
        let mut new: Vec<Option<Point>> = Vec::new();

        let none_count = vec.iter().filter(|p| p.is_none()).count();
        for _ in 0..none_count {
            new.push(None);
        }

        for i in vec{
            match i {
                None => {}
                Some(_) => new.push(i.clone())
            }
        }

        self.body = new
    }
    pub fn eat_itself(&self) -> bool {
        for (i, point) in self.body.iter().enumerate() {
            if point.is_some() {
                for point2 in &self.body[0..i] {
                    if point2.is_some() && point == point2 {
                        return true
                    }
                }
            }
        }
        false
    }
    pub fn is_outside(&self, board: &Board) -> bool {
        let pos = self.get_head();
        pos.x < 1 || pos.y < 1 ||
            pos.x >= board.width()-1 || pos.y >= board.height()-1
    }
    pub fn has_apple(&self, apple: &Option<Point>) -> bool {
        match apple{
            None => false,
            Some(p) => p == self.get_head()
        }
    }
    pub fn draw(&self, board: &mut Board, body_color: Color){
        for i in &self.body {
            match i {
                Some(pos) => board.draw_cell(pos.x, pos.y, body_color.clone()),
                None => {}
            }
        }
    }
    pub fn add_body_size(&mut self, size: i32){
        for _ in 0..size {
            self.body.push(None);
        }
        self.organise_player();
    }
}

impl Game{
    pub fn new() -> Self {

        let n_cells = 50;

        let window = Window::new(700, 1200, "Snake".into());
        let board = Board::new(window, n_cells);

        let player = Player::new_local("local".into(), Point::new(board.width()/2, board.height()/2));

        Self{
            board,
            apple: None,
            player,
            running: true,
            players: Vec::new()
        }
    }


    pub fn update(&mut self){
        self.board.handle_events();
        let dir = self.board.get_next_direction();
        self.player.forward(dir);

        if self.player.has_apple(&self.apple){
            self.apple = None;
            self.player.add_body_size(5);
        }

        if self.apple.is_none(){
            self.apple = Some(self.generate_apple_pos());
        }

        if self.board.should_close() {
            self.running = false;
        }

        if self.player.eat_itself() || self.player.is_outside(&self.board) {
            self.running = false
        }

    }
    pub fn draw(&mut self){
        let background_color = Color::new(0,0,0);
        let body_color = Color::new(0,255,0);
        let border_color = Color::new(0,0,255);
        let apple_color = Color::new(255, 0, 0);

        self.board.clear(background_color);
        self.board.draw_borders(border_color);

        // Draw players
        self.player.draw(&mut self.board, body_color.clone());
        for player in &self.players {
            player.draw(&mut self.board, body_color.clone());
        }

        // Draw apple
        match &self.apple {
            None => {}
            Some(pos) => self.board.draw_cell(pos.x, pos.y, apple_color)
        }

        self.board.draw();
    }
    pub fn running(&self) -> bool {
        self.running
    }
    pub fn generate_apple_pos(&self) -> Point{
        let x: i32 = (rand::random::<u16>() as i32)%(self.board.width()-2)+1;
        let y: i32 = (rand::random::<u16>() as i32)%(self.board.height()-2)+1;

        Point::new(x, y)
    }
}