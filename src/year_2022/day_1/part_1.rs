use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "69289"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut max = 0;
        let mut current = 0;

        for line in lines {
            if line.is_empty() {
                max = current.max(max);
                current = 0;
                continue;
            }

            current += line.parse::<u32>().unwrap();
        }

        max.to_string()
    }
}
