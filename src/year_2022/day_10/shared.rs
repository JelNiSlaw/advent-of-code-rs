pub enum Instruction {
    AddX(i32),
    NoOp,
}

pub fn parse_instructions(lines: Vec<String>) -> impl Iterator<Item = Instruction> {
    lines.into_iter().flat_map(|line| {
        let mut chars = line.chars();

        match chars
            .by_ref()
            .take_while(|c| !c.is_ascii_whitespace())
            .collect::<String>()
            .as_str()
        {
            "addx" => {
                let value = chars.by_ref().collect::<String>().parse::<i32>().unwrap();
                vec![Instruction::NoOp, Instruction::AddX(value)]
            }
            "noop" => vec![Instruction::NoOp],
            _ => unreachable!(),
        }
    })
}
