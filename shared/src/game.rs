
#[derive(PartialEq)]
pub enum Direction{
    UP,
    LEFT,
    DOWN,
    RIGHT
}
impl Direction{
    pub fn opposite(&self, other_direction: &Direction) -> bool {
        use Direction::*;

        match (self, other_direction){
            (UP, DOWN) => true,
            (DOWN, UP) => true,
            (LEFT, RIGHT) => true,
            (RIGHT, LEFT) => true,
            _ => false
        }
    }
}
