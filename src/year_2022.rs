use advent_of_code::Solve;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

pub fn days() -> Vec<Vec<Box<dyn Solve>>> {
    vec![
        day_1::parts(),
        day_2::parts(),
        day_3::parts(),
        day_4::parts(),
        day_5::parts(),
        day_6::parts(),
    ]
}
