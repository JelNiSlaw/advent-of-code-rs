use advent_of_code::Solve;

mod day_1;

pub fn days() -> Vec<Vec<Box<dyn Solve>>> {
    vec![day_1::parts()]
}
