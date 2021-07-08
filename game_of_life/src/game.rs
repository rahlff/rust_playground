use array2d::Array2D;
use rand::prelude::*;

use crate::cell::Cell;

pub struct Game {
    height: usize,
    width: usize,
    grid: Array2D<Cell>,
}

impl Game {
    pub fn new(height: usize, width: usize) -> Game {
        let mut game = Game {
            grid: Array2D::filled_with(Cell::Empty, height, width),
            height,
            width,
        };
        
        // starting position cannot be at the egde. So we get a random position 10 position from
        // the edge
        let starting_x_pos = width / 2;//rand::thread_rng().gen_range(10..width-10);
        let starting_y_pos = height / 2;//rand::thread_rng().gen_range(10..height-10);

        // Start with random cells around the starting pos
        // ? | ? | ?
        // ? | X | ?
        // ? | ? | ?

        /*
        for x in 0..3 {
            for y in 0..3 {
                game.grid.get_mut(starting_x_pos-1+x, starting_y_pos-1+y).map(|x| *x = rand::random());
            }
        }
        */

        game.grid[(starting_x_pos,starting_y_pos)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+1)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+2)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+3)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+4)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+5)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+6)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+7)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+8)] = Cell::Alive;
        game.grid[(starting_x_pos,starting_y_pos+9)] = Cell::Alive;
        game
    }

    pub fn get_cell(& self, x: usize, y: usize) -> &Cell
    {
        &self.grid[(x,y)]
    }

    pub fn update(&mut self) {

        let mut alive_neighbor: u8;

        for x in 0..self.width {
            for y in 0..self.height {
                alive_neighbor = self.get_alive_neighbor_count(x, y);

                match self.grid.get(x,y) {
                    Some(cell_ref) => {
                        if *cell_ref == Cell::Alive {
                            if alive_neighbor < 2 {
                                self.grid[(x,y)] = Cell::Empty;
                            } else if alive_neighbor == 3 {
                                self.grid[(x,y)] = Cell::Alive;
                            } else if alive_neighbor == 2 {
                                self.grid[(x,y)] = Cell::Alive;
                            } else if alive_neighbor > 3 {
                                self.grid[(x,y)] = Cell::Empty;
                            }
                        }
                        else {
                            if alive_neighbor == 3 {
                                self.grid[(x,y)] = Cell::Alive;
                            }
                        }
                    },
                    None => {/* out of bounds*/},
                }
            }
        }
    }

    fn get_alive_neighbor_count(&self, x_pos: usize, y_pos: usize) -> u8 {
        let mut alive: u8 = 0;
        for x in 0..3 {
            if x_pos+x == 0 {
                continue;
            }
            for y in 0..3 {
                if y_pos+y == 0 {
                    continue;
                }
                match self.grid.get(x_pos+x-1, y_pos+y-1) {
                    Some(x) => {
                        if *x == Cell::Alive {
                            alive += 1;
                        }
                    },
                    None => {/* out of bounds*/},
                }
            }
        }
        alive
    }
}
