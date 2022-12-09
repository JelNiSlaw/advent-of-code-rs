use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1233"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut count = 0;
        let mut last_depth = None;
        for line in lines {
            let depth = line.parse::<u32>().unwrap();

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
