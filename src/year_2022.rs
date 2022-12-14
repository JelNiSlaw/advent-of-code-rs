use advent_of_code::Solve;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

pub fn days() -> Vec<Vec<Box<dyn Solve>>> {
    vec![
        day_1::parts(),
        day_2::parts(),
        day_3::parts(),
        day_4::parts(),
        day_5::parts(),
        day_6::parts(),
        day_7::parts(),
        day_8::parts(),
        day_9::parts(),
        day_10::parts(),
        day_11::parts(),
        day_12::parts(),
        day_13::parts(),
        day_14::parts(),
    ]
}
