#[allow(clippy::needless_pass_by_value)]
pub fn print_answer<S: ToString>(year: u32, day: u32, part: u32, solution: S) {
    println!(
        "year {year}, day {day}, part {part}: {}",
        solution.to_string()
    );
}
