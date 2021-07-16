use sdl2::rect::Point;
use crate::direction::Direction;

pub struct Walker {
    pub pos: Point,
}

impl Walker {
    pub fn new(x:i32, y:i32) -> Walker {
        let walker = Walker {
            pos : Point::new(x,y),
        };
        walker
    }

    pub fn update(&mut self) {
        let direction: Direction = rand::random();
        match direction {
            Direction::Forward => {
                // forward is up
                self.pos.y -= 1;
            },
            Direction::Backward =>{
                self.pos.y += 1;
            },
            Direction::Left =>{
                self.pos.x -= 1;
            },
            Direction::Right =>{
                self.pos.x += 1;
            },
        }

    }
}
