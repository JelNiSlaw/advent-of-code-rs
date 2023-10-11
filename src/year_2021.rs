use advent_of_code::Solve;

mod day_1;
mod day_2;
mod day_3;

pub fn days() -> Vec<Vec<Box<dyn Solve>>> {
    vec![day_1::parts(), day_2::parts(), day_3::parts()]
}
