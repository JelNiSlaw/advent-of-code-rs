use advent_of_code::Solve;

use super::shared::Monkey;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "62491"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut lines = lines.into_iter();

        let mut monkeys = Vec::new();
        loop {
            let lines = lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .collect::<Vec<_>>();

            if lines.is_empty() {
                break;
            }

            monkeys.push(Monkey::parse(lines.into_iter()));
        }

        for _ in 0..20 {
            let mut m = 0;
            while m < monkeys.len() {
                let monkey = monkeys.get_mut(m).unwrap();
                let items = monkey.items.drain(..).collect::<Vec<_>>();
                monkey.inspected_items += items.len();
                let operation = monkey.operation;
                let test = monkey.test;
                for item in items {
                    let item = operation.calculate(item) / 3;
                    let pass_to = test.test(item);
                    let monkey = monkeys.get_mut(pass_to).unwrap();
                    monkey.items.push(item);
                }

                m += 1;
            }
        }

        monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));
        (monkeys[0].inspected_items * monkeys[1].inspected_items).to_string()
    }
}