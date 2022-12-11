use advent_of_code::Solve;

mod part_1;
mod part_2;
mod shared;

pub fn parts() -> Vec<Box<dyn Solve>> {
    vec![Box::new(part_1::Solution), Box::new(part_2::Solution)]
}
