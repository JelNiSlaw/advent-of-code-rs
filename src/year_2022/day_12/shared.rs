use std::collections::{HashSet, VecDeque};

pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub depth: u32,
}

pub fn breadth_first_search(lines: impl Iterator<Item = String>, start: &[char], end: char) -> u32 {
    let mut queue = VecDeque::new();

    let grid = lines
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if start.contains(&char) {
                        queue.push_back(Tile { x, y, depth: 0 });
                        'a'
                    } else {
                        char
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visited = queue
        .iter()
        .map(|item| (item.x, item.y))
        .collect::<HashSet<_>>();

    while let Some(tile) = queue.pop_front() {
        let current_value = grid[tile.y][tile.x];

        if current_value == end {
            return tile.depth;
        }

        let mut to_check = Vec::with_capacity(4);

        if tile.y > 0 {
            to_check.push((tile.x, tile.y - 1));
        }

        if tile.x + 1 < grid[tile.y].len() {
            to_check.push((tile.x + 1, tile.y));
        }

        if tile.y + 1 < grid.len() {
            to_check.push((tile.x, tile.y + 1));
        }

        if tile.x > 0 {
            to_check.push((tile.x - 1, tile.y));
        }

        for (x, y) in to_check {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut option = grid[y][x];
            if option == end {
                option = 'z';
            }

            if option as u32 > current_value as u32 + 1 {
                continue;
            }

            queue.push_back(Tile {
                x,
                y,
                depth: tile.depth + 1,
            });

            visited.insert((x, y));
        }
    }

    unreachable!()
}
