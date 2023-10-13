use itertools::Itertools;

pub struct Instruction {
    pub move_amount: usize,
    pub move_from: usize,
    pub move_to: usize,
}

pub fn parse_stacks(mut lines: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        for (i, mut item) in line.chars().chunks(4).into_iter().enumerate() {
            if i >= stacks.len() {
                stacks.push(Vec::new());
            }

            if item.by_ref().next().unwrap() == '[' {
                stacks[i].insert(0, item.by_ref().next().unwrap());
            }
        }
    }

    stacks
}

pub fn parse_instructions(
    lines: impl Iterator<Item = String>,
) -> impl Iterator<Item = Instruction> {
    lines.map(|line| {
        let mut chars = line.chars();

        let move_amount = chars
            .by_ref()
            .skip("move ".len())
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let move_from = chars
            .by_ref()
            .skip("from ".len())
            .take_while(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            - 1;

        let move_to = chars
            .by_ref()
            .skip("to ".len())
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
            - 1;

        Instruction {
            move_amount,
            move_from,
            move_to,
        }
    })
}
