pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test: Test,
    pub inspected_items: usize,
}

impl Monkey {
    pub fn parse(lines: impl Iterator<Item = String>) -> Self {
        let mut lines = lines.skip(1);
        let items = lines
            .next()
            .unwrap()
            .chars()
            .skip(18)
            .collect::<String>()
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let operation = Operation::parse(&lines.next().unwrap());

        let test = Test::parse(lines);

        Self {
            items,
            operation,
            test,
            inspected_items: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Operation {
    Add(Value),
    MultiplyBy(Value),
}

impl Operation {
    fn parse(line: &str) -> Self {
        let mut chars = line.chars().skip(23);
        let operator = chars.by_ref().next().unwrap();

        let value = match chars.by_ref().skip(1).collect::<String>().as_str() {
            "old" => Value::Current,
            number => Value::Value(number.parse().unwrap()),
        };

        match operator {
            '+' => Self::Add(value),
            '*' => Self::MultiplyBy(value),
            _ => unreachable!(),
        }
    }

    pub fn calculate(&self, current_value: u64) -> u64 {
        match self {
            Self::Add(value) => match value {
                Value::Value(value) => current_value + value,
                Value::Current => current_value * 2,
            },
            Self::MultiplyBy(value) => match value {
                Value::Value(value) => current_value * value,
                Value::Current => current_value.pow(2),
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum Value {
    Value(u64),
    Current,
}

#[derive(Clone, Copy)]
pub struct Test {
    pub divisible_by: u64,
    pub if_true: usize,
    pub if_false: usize,
}

impl Test {
    fn parse(mut lines: impl Iterator<Item = String>) -> Self {
        let divisible_by = lines
            .next()
            .unwrap()
            .chars()
            .skip(21)
            .collect::<String>()
            .parse()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .chars()
            .skip(29)
            .collect::<String>()
            .parse()
            .unwrap();

        let if_false = lines
            .next()
            .unwrap()
            .chars()
            .skip(30)
            .collect::<String>()
            .parse()
            .unwrap();

        Self {
            divisible_by,
            if_true,
            if_false,
        }
    }

    pub const fn test(&self, value: u64) -> usize {
        if value % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}
