use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "770"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let count = lines
            .into_iter()
            .filter(|line| {
                let mut chars = line.chars();
                let start_a = chars
                    .by_ref()
                    .take_while(|c| *c != '-')
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                let end_a = chars
                    .by_ref()
                    .take_while(|c| *c != ',')
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                let start_b = chars
                    .by_ref()
                    .take_while(|c| *c != '-')
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                let end_b = chars.by_ref().collect::<String>().parse::<u32>().unwrap();

                start_a <= end_b && start_b <= end_a
            })
            .count();

        count.to_string()
    }
}
