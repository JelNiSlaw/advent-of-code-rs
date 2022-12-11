use std::mem;

use advent_of_code::Solve;
use itertools::Itertools;

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
                let items = mem::take(&mut monkey.items);
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

        monkeys
            .into_iter()
            .map(|monkey| monkey.inspected_items)
            .sorted_by(|a, b| b.cmp(a))
            .take(2)
            .product::<usize>()
            .to_string()
    }
}
