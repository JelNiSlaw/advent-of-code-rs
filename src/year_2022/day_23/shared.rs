use std::collections::HashSet;

pub const DIRECTIONS: [Direction; 4] = [
    Direction {
        delta: (0, -1),
        checks: [(0, -1), (1, -1), (-1, -1)],
    },
    Direction {
        delta: (0, 1),
        checks: [(0, 1), (1, 1), (-1, 1)],
    },
    Direction {
        delta: (-1, 0),
        checks: [(-1, 0), (-1, -1), (-1, 1)],
    },
    Direction {
        delta: (1, 0),
        checks: [(1, 0), (1, -1), (1, 1)],
    },
];

#[derive(Clone, Copy)]
pub struct Direction {
    pub delta: (i32, i32),
    pub checks: [(i32, i32); 3],
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
    if [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ]
    .into_iter()
    .all(|check| !elves.contains(&(elve.0 + check.0, elve.1 + check.1)))
    {
        return None;
    }

    directions
        .into_iter()
        .filter(|direction| {
            direction
                .checks
                .into_iter()
                .all(|check| !elves.contains(&(elve.0 + check.0, elve.1 + check.1)))
        })
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
