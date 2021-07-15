use crate::direction::Direction;

pub struct Walker {
    pub x: u16,
    pub y: u16,
}

impl Walker {
    pub fn new(x:u16, y:u16) -> Walker {
        let walker = Walker {
            x,
            y,
        };
        walker
    }

    pub fn update(&mut self) {
        let direction: Direction = rand::random();
        match direction {
            Direction::Forward => {
                // forward is up
                self.y -= 1;
            },
            Direction::Backward =>{
                self.y += 1;
            },
            Direction::Left =>{
                self.x -= 1;
            },
            Direction::Right =>{
                self.x += 1;
            },
        }

    }
}
