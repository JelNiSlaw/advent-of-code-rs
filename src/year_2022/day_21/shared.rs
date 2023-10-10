use std::collections::HashMap;

pub type Monkeys = HashMap<String, Operation>;

#[derive(Clone, Debug)]
pub enum Operation {
    Value(u64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Equals(String, String),
    Unknown,
}

impl Operation {
    fn parse(mut chars: impl Iterator<Item = char>) -> Self {
        let first_char = chars.by_ref().next().unwrap();
        if first_char.is_ascii_digit() {
            let mut string = first_char.to_string();
            string.extend(chars);
            Self::Value(string.parse().unwrap())
        } else {
            let mut lhs = first_char.to_string();
            lhs.extend(chars.by_ref().take_while(char::is_ascii_alphabetic));
            let operator = chars.by_ref().next().unwrap();
            let rhs = chars.by_ref().skip(1).collect::<String>();

            match operator {
                '+' => Self::Add(lhs, rhs),
                '-' => Self::Subtract(lhs, rhs),
                '*' => Self::Multiply(lhs, rhs),
                '/' => Self::Divide(lhs, rhs),
                _ => unreachable!(),
            }
        }
    }
}

pub fn parse_monkeys(lines: impl Iterator<Item = String>) -> Monkeys {
    lines
        .map(|line| {
            let mut chars = line.chars();
            let name = chars
                .by_ref()
                .take_while(char::is_ascii_alphabetic)
                .collect::<String>();

            let operation = Operation::parse(chars.skip(1));

            (name, operation)
        })
        .collect()
}

pub fn monkey_value(monkey_name: &str, monkeys: &mut Monkeys) -> Option<u64> {
    let value = match monkeys.get(monkey_name).unwrap().clone() {
        Operation::Value(value) => return Some(value),
        Operation::Add(lhs, rhs) => monkey_value(&lhs, monkeys)? + monkey_value(&rhs, monkeys)?,
        Operation::Subtract(lhs, rhs) => {
            monkey_value(&lhs, monkeys)? - monkey_value(&rhs, monkeys)?
        }
        Operation::Multiply(lhs, rhs) => {
            monkey_value(&lhs, monkeys)? * monkey_value(&rhs, monkeys)?
        }
        Operation::Divide(lhs, rhs) => monkey_value(&lhs, monkeys)? / monkey_value(&rhs, monkeys)?,
        Operation::Equals(lhs, rhs) => {
            match (monkey_value(&lhs, monkeys), monkey_value(&rhs, monkeys)) {
                (Some(value), None) => resolve_unknown(rhs, value, monkeys),
                (None, Some(value)) => resolve_unknown(lhs, value, monkeys),
                _ => unreachable!(),
            }
            return None;
        }
        Operation::Unknown => return None,
    };

    monkeys.insert(monkey_name.into(), Operation::Value(value));

    Some(value)
}

pub fn resolve_unknown(monkey_name: String, expected_value: u64, monkeys: &mut Monkeys) {
    let (monkey_name, value) = match monkeys.get(&monkey_name).unwrap().clone() {
        Operation::Add(lhs, rhs) => {
            match (monkey_value(&lhs, monkeys), monkey_value(&rhs, monkeys)) {
                (Some(value), None) => (rhs, expected_value - value),
                (None, Some(value)) => (lhs, expected_value - value),
                _ => unreachable!(),
            }
        }
        Operation::Subtract(lhs, rhs) => {
            match (monkey_value(&lhs, monkeys), monkey_value(&rhs, monkeys)) {
                (Some(value), None) => (rhs, value - expected_value),
                (None, Some(value)) => (lhs, value + expected_value),
                _ => unreachable!(),
            }
        }
        Operation::Multiply(lhs, rhs) => {
            match (monkey_value(&lhs, monkeys), monkey_value(&rhs, monkeys)) {
                (Some(value), None) => (rhs, expected_value / value),
                (None, Some(value)) => (lhs, expected_value / value),
                _ => unreachable!(),
            }
        }
        Operation::Divide(lhs, rhs) => {
            match (monkey_value(&lhs, monkeys), monkey_value(&rhs, monkeys)) {
                (Some(value), None) => (rhs, value / expected_value),
                (None, Some(value)) => (lhs, value * expected_value),
                _ => unreachable!(),
            }
        }
        Operation::Unknown => {
            monkeys.insert(monkey_name, Operation::Value(expected_value));
            return;
        }
        _ => unreachable!(),
    };

    resolve_unknown(monkey_name, value, monkeys);
}
