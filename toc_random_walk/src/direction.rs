use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Right,
    Left,
}

// This is heavy inspired from
// https://stackoverflow.com/questions/48490049/how-do-i-choose-a-random-value-from-an-enum
impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Forward,
            1 => Direction::Backward,
            2 => Direction::Right,
            3 => Direction::Left,
        }
    }
}
