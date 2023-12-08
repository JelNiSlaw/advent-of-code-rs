use std::collections::HashMap;

type Node = [u8; 3];

#[derive(Clone, Copy)]
enum Instruction {
    L,
    R,
}

impl Instruction {
    fn parse(byte: u8) -> Self {
        match byte {
            b'L' => Self::L,
            b'R' => Self::R,
            _ => unreachable!(),
        }
    }
}

pub struct Map {
    instructions: Vec<Instruction>,
    nodes: HashMap<Node, (Node, Node)>,
}

impl Map {
    pub fn parse(lines: Vec<String>) -> Self {
        let mut lines = lines.into_iter();

        let instructions = lines
            .by_ref()
            .next()
            .unwrap()
            .bytes()
            .map(Instruction::parse)
            .collect();

        let nodes = lines
            .skip(1)
            .map(|line| {
                let mut line = line.bytes();

                let current = line.by_ref().take(3).collect::<Vec<_>>();
                let left = line.by_ref().skip(" = (".len()).take(3).collect::<Vec<_>>();
                let right = line.skip(", ".len()).take(3).collect::<Vec<_>>();

                (
                    current.try_into().unwrap(),
                    (left.try_into().unwrap(), right.try_into().unwrap()),
                )
            })
            .collect();

        Self {
            instructions,
            nodes,
        }
    }

    pub fn path_length(&self, start: Node, end_condition: impl Fn(Node) -> bool) -> u64 {
        let mut node = start;
        let mut instruction_pointer = 0;

        loop {
            let instruction = self.instructions[instruction_pointer % self.instructions.len()];

            let current_node = self.nodes[&node];

            node = match instruction {
                Instruction::L => current_node.0,
                Instruction::R => current_node.1,
            };

            instruction_pointer += 1;

            if end_condition(node) {
                break;
            }
        }

        instruction_pointer.try_into().unwrap()
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.nodes.keys()
    }
}
