use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Alive,
    Dying,
}

// This is heavy inspired from
// https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
impl Distribution<Cell> for Standard {
    
    // This sample implementation does NOT return a random Cell value choosen from all 3 Cell values
    // The Cell value Dying is an temperaly Cell state, which is ommitted in the random Cell generation.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0..=1) {
            0 => Cell::Empty,
            _ => Cell::Alive,
        }
    }
}
