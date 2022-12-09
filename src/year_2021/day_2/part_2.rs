use advent_of_code::Solve;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1781819478"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut position = 0;
        let mut depth = 0;
        let mut aim = 0;

        for line in lines {
            let mut chars = line.chars();
            let instruction = chars
                .by_ref()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect::<String>();
            let amount = chars.by_ref().collect::<String>().parse::<u32>().unwrap();

            match instruction.as_str() {
                "down" => aim += amount,
                "up" => aim -= amount,
                "forward" => {
                    position += amount;
                    depth += aim * amount;
                }
                _ => unreachable!(),
            }
        }

        (position * depth).to_string()
    }
}
