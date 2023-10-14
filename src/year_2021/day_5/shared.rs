use std::collections::HashMap;

pub enum Direction {
    Horizontal,
    Vertical,
    DiagonalPositive,
    DiagonalNegative,
}

pub struct Line {
    pub x: u32,
    pub y: u32,
    pub length: u32,
    pub direction: Direction,
}

impl Line {
    pub fn parse(line: &str) -> Self {
        let mut chars = line.chars();
        let (x1, y1) = parse_xy(&mut chars);
        let (x2, y2) = parse_xy(&mut chars.skip("-> ".len()));

        let horizontal_distance = x1.abs_diff(x2);
        let vertical_distance = y1.abs_diff(y2);

        let (x, len, direction) = if y1 == y2 {
            (x1.min(x2), horizontal_distance, Direction::Horizontal)
        } else if x1 == x2 {
            (x1.min(x2), vertical_distance, Direction::Vertical)
        } else if (x1 < x2) ^ (y1 > y2) {
            (x1.min(x2), horizontal_distance, Direction::DiagonalPositive)
        } else {
            (x1.max(x2), horizontal_distance, Direction::DiagonalNegative)
        };

        let y = y1.min(y2);

        Self {
            x,
            y,
            length: len,
            direction,
        }
    }

    pub fn positions<'a>(&'a self) -> Box<dyn Iterator<Item = (u32, u32)> + 'a> {
        match self.direction {
            Direction::Horizontal => {
                Box::new((0..=self.length).map(|offset| (self.x + offset, self.y)))
            }
            Direction::Vertical => {
                Box::new((0..=self.length).map(|offset| (self.x, self.y + offset)))
            }
            Direction::DiagonalPositive => {
                Box::new((0..=self.length).map(|offset| (self.x + offset, self.y + offset)))
            }
            Direction::DiagonalNegative => {
                Box::new((0..=self.length).map(|offset| (self.x - offset, self.y + offset)))
            }
        }
    }
}

fn parse_xy(mut chars: impl Iterator<Item = char>) -> (u32, u32) {
    let x = chars
        .by_ref()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    let y = chars
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    (x, y)
}

pub fn evaluate_ocean_floor(
    lines: impl Iterator<Item = Line>,
    ignore_diagonals: bool,
) -> HashMap<(u32, u32), u32> {
    let mut ocean_floor = HashMap::new();

    for line in lines {
        if ignore_diagonals
            && !matches!(line.direction, Direction::Horizontal | Direction::Vertical)
        {
            continue;
        }

        for position in line.positions() {
            #[allow(clippy::option_if_let_else)]
            match ocean_floor.get_mut(&position) {
                Some(value) => *value += 1,
                None => {
                    ocean_floor.insert(position, 1);
                }
            };
        }
    }

    ocean_floor
}
