use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "12989"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        #[allow(clippy::identity_op)]
        let points = lines
            .into_iter()
            .map(|round| match round.as_ref() {
                "A X" => 3 + 0,
                "A Y" => 1 + 3,
                "A Z" => 2 + 6,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 2 + 0,
                "C Y" => 3 + 3,
                "C Z" => 1 + 6,
                _ => unreachable!(),
            })
            .sum::<u32>();

        points.to_string()
    }
}
