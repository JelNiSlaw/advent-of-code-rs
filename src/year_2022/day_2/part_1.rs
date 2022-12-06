use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "15691"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        #[allow(clippy::identity_op)]
        let points = lines
            .into_iter()
            .map(|round| match round.as_ref() {
                "A X" => 1 + 3,
                "A Y" => 2 + 6,
                "A Z" => 3 + 0,
                "B X" => 1 + 0,
                "B Y" => 2 + 3,
                "B Z" => 3 + 6,
                "C X" => 1 + 6,
                "C Y" => 2 + 0,
                "C Z" => 3 + 3,
                _ => unreachable!(),
            })
            .sum::<u32>();

        points.to_string()
    }
}
