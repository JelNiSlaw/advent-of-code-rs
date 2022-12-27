use std::collections::HashSet;

pub const DIRECTIONS: [Direction; 4] = [
    Direction {
        delta: (0, -1),
        checks: [0, 1, 7], // N, NE, NW
    },
    Direction {
        delta: (0, 1),
        checks: [4, 3, 5], // S, SE, SW
    },
    Direction {
        delta: (-1, 0),
        checks: [6, 7, 5], // W, NW, SW
    },
    Direction {
        delta: (1, 0),
        checks: [2, 1, 3], // E, NE, SE
    },
];

#[derive(Clone, Copy)]
pub struct Direction {
    pub delta: (i32, i32),
    pub checks: [usize; 3],
}

pub fn parse_elves(lines: impl Iterator<Item = String>) -> HashSet<(i32, i32)> {
    lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, char)| char == '#')
                .map(|(x, _)| (x.try_into().unwrap(), y.try_into().unwrap()))
                .collect::<HashSet<_>>()
        })
        .collect()
}

fn propose(
    elve: (i32, i32),
    directions: [Direction; 4],
    elves: &HashSet<(i32, i32)>,
) -> Option<(i32, i32)> {
    let checks = [
        (0, -1),  // N
        (1, -1),  // NE
        (1, 0),   // E
        (1, 1),   // SE
        (0, 1),   // S
        (-1, 1),  // SW
        (-1, 0),  // W
        (-1, -1), // NW
    ]
    .into_iter()
    .map(|check| !elves.contains(&(elve.0 + check.0, elve.1 + check.1)))
    .collect::<Vec<_>>();

    if checks.iter().all(|&check| check) {
        return None;
    }

    directions
        .into_iter()
        .filter(|direction| direction.checks.into_iter().all(|check| checks[check]))
        .map(|direction| (elve.0 + direction.delta.0, elve.1 + direction.delta.1))
        .next()
}

pub fn diffuse(elves: &HashSet<(i32, i32)>, directions: [Direction; 4]) -> HashSet<(i32, i32)> {
    let propositions = elves
        .iter()
        .filter_map(|&elve| propose(elve, directions, elves))
        .collect::<Vec<_>>();

    elves
        .iter()
        .map(|&elve| {
            propose(elve, directions, elves).map_or(elve, |proposition| {
                if propositions.iter().filter(|&&x| x == proposition).count() > 1 {
                    elve
                } else {
                    proposition
                }
            })
        })
        .collect()
}
