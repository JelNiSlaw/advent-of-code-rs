use std::collections::HashSet;

use advent_of_code::Solve;

use super::shared::parse_line;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "27936"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let grid = lines
            .into_iter()
            .flat_map(|line| parse_line(&line))
            .collect::<HashSet<_>>();

        simulate_sand(grid, (500, 0)).to_string()
    }
}

fn simulate_sand(mut grid: HashSet<(u32, u32)>, sand_source: (u32, u32)) -> u32 {
    let lowest_platform = grid.iter().map(|point| point.1).max().unwrap();

    let mut i = 0;
    while !grid.contains(&sand_source) {
        i += 1;
        let mut grain = sand_source;
        while grain.1 <= lowest_platform {
            if grid.contains(&(grain.0, grain.1 + 1)) {
                if !grid.contains(&(grain.0 - 1, grain.1 + 1)) {
                    grain.0 -= 1;
                } else if !grid.contains(&(grain.0 + 1, grain.1 + 1)) {
                    grain.0 += 1;
                } else {
                    grid.insert(grain);
                    break;
                }
            }

            grain.1 += 1;
        }

        grid.insert(grain);
    }

    i
}
