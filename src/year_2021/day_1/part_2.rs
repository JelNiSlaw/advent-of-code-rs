use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1275"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut count = 0;
        let mut last_depth = None;
        for lines in lines.windows(3) {
            let depth = lines
                .iter()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>();

            if let Some(last_depth) = last_depth {
                if depth > last_depth {
                    count += 1;
                }
            }

            last_depth = Some(depth);
        }

        count.to_string()
    }
}
