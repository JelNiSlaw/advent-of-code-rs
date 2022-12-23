use advent_of_code::Solve;

use super::shared::{monkey_value, parse_monkeys, Operation};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "3327575724809"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let mut monkeys = parse_monkeys(lines.into_iter());

        match monkeys.remove("root").unwrap() {
            Operation::Add(lhs, rhs)
            | Operation::Subtract(lhs, rhs)
            | Operation::Multiply(lhs, rhs)
            | Operation::Divide(lhs, rhs) => {
                monkeys.insert("root".to_string(), Operation::Equals(lhs, rhs));
            }
            _ => unreachable!(),
        };

        monkeys.insert("humn".into(), Operation::Unknown);

        monkey_value("root", &mut monkeys);
        if let Operation::Value(value) = monkeys.get("humn").unwrap() {
            value.to_string()
        } else {
            unreachable!()
        }
    }
}
