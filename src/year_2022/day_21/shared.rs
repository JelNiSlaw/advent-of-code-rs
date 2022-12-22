use std::collections::HashMap;

#[derive(Clone)]
pub enum Operation {
    Value(u64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
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
                '+' => Operation::Add(lhs, rhs),
                '-' => Operation::Subtract(lhs, rhs),
                '*' => Operation::Multiply(lhs, rhs),
                '/' => Operation::Divide(lhs, rhs),
                _ => unreachable!(),
            }
        }
    }
}

pub fn parse_monkeys(lines: impl Iterator<Item = String>) -> HashMap<String, Operation> {
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

pub fn monkey_value(monkey_name: &str, monkeys: &mut HashMap<String, Operation>) -> u64 {
    let value = match monkeys.get(monkey_name).unwrap().clone() {
        Operation::Value(value) => return value,
        Operation::Add(lhs, rhs) => monkey_value(&lhs, monkeys) + monkey_value(&rhs, monkeys),
        Operation::Subtract(lhs, rhs) => monkey_value(&lhs, monkeys) - monkey_value(&rhs, monkeys),
        Operation::Multiply(lhs, rhs) => monkey_value(&lhs, monkeys) * monkey_value(&rhs, monkeys),
        Operation::Divide(lhs, rhs) => monkey_value(&lhs, monkeys) / monkey_value(&rhs, monkeys),
    };

    monkeys.insert(monkey_name.into(), Operation::Value(value));

    value
}
