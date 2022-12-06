use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "205615"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut elves = vec![];
        let mut current = 0;

        for line in lines {
            if line.is_empty() {
                elves.push(current);
                current = 0;
                continue;
            }

            current += line.parse::<u32>().unwrap();
        }

        elves.sort_by(|a, b| b.cmp(a));
        let top_sum = elves.into_iter().take(3).sum::<u32>();

        top_sum.to_string()
    }
}
