use std::cmp::Ordering;

#[derive(Clone, PartialEq)]
pub enum Data {
    Int(u32),
    List(Vec<Data>),
}

impl Data {
    pub fn parse(line: &str) -> Self {
        let mut list = Vec::new();
        let mut chars = line.chars();

        while let Some(char) = chars.by_ref().next() {
            match char {
                '[' => list.push(Self::parse(
                    &chars.by_ref().take_while(|c| *c != ']').collect::<String>(),
                )),
                ']' | ',' => (),
                _ => {
                    let mut number = char.to_string();
                    number.extend(chars.by_ref().take_while(char::is_ascii_digit));
                    list.push(Self::Int(number.parse().unwrap()));
                }
            }
        }

        Self::List(list)
    }
}

pub fn right_order(left: &Data, right: &Data) -> Option<bool> {
    match (left, right) {
        (Data::Int(left), Data::Int(right)) => match left.cmp(right) {
            Ordering::Less => Some(true),
            Ordering::Equal => None,
            Ordering::Greater => Some(false),
        },
        (Data::List(left), Data::List(right)) => {
            let left_shorter = left.len() < right.len();
            let right_shorter = right.len() < left.len();

            for (left, right) in left.iter().zip(right) {
                let result = right_order(left, right);
                if result.is_some() {
                    return result;
                }
            }

            if left_shorter {
                Some(true)
            } else if right_shorter {
                Some(false)
            } else {
                None
            }
        }
        (left, Data::Int(right)) => right_order(left, &Data::List(vec![Data::Int(*right)])),
        (Data::Int(left), right) => right_order(&Data::List(vec![Data::Int(*left)]), right),
    }
}
