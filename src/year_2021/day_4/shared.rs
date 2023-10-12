#[derive(Clone, Copy)]
pub struct Board {
    numbers: [[(u8, bool); 5]; 5],
}

impl Board {
    pub fn parse(lines: impl Iterator<Item = String>) -> Self {
        let mut numbers = [[(0, false); 5]; 5];

        for (row, line) in lines.into_iter().enumerate() {
            for (column, number) in line
                .split(' ')
                .filter(|number| !number.is_empty())
                .enumerate()
            {
                numbers[row][column].0 = number.parse().unwrap();
            }
        }

        Self { numbers }
    }

    pub fn mark(&mut self, number: u8) -> bool {
        for row in &mut self.numbers {
            for cell in row.iter_mut() {
                if cell.0 == number {
                    cell.1 = true;
                    return true;
                }
            }
        }

        false
    }

    pub fn is_winning(&self) -> bool {
        for row in self.numbers {
            if row.into_iter().all(|(_, marked)| marked) {
                return true;
            }
        }

        for column in 0..self.numbers[0].len() {
            if (0..self.numbers.len()).all(|row| self.numbers[row][column].1) {
                return true;
            }
        }

        false
    }

    pub fn score(&self) -> u32 {
        self.numbers
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .filter(|cell| !cell.1)
                    .map(|cell| u32::from(cell.0))
                    .sum::<u32>()
            })
            .sum()
    }
}

pub fn parse_input(lines: Vec<String>) -> (Vec<u8>, Vec<Board>) {
    let mut lines = lines.into_iter().peekable();

    let winning_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    lines.next().unwrap();

    let mut boards = Vec::new();

    loop {
        if lines.peek().is_none() {
            break;
        }

        boards.push(Board::parse(
            lines.by_ref().take_while(|line| !line.is_empty()),
        ));
    }

    (winning_numbers, boards)
}
