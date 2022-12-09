use advent_of_code::Solve;

use super::shared::get_directions;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "1647"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let forest = lines
            .into_iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut count = 0;
        for row in 1..forest.len() - 1 {
            for collumn in 1..forest[row].len() - 1 {
                if is_tree_visible((row, collumn), &forest) {
                    count += 1;
                }
            }
        }

        count += (forest.len() - 1) * 2 + (forest[0].len() - 1) * 2;

        count.to_string()
    }
}

fn is_tree_visible(location: (usize, usize), forest: &[Vec<u32>]) -> bool {
    let tree_height = forest[location.0][location.1];

    get_directions(location, forest)
        .into_iter()
        .filter_map(|trees| trees.iter().max().copied())
        .any(|max_height| max_height < tree_height)
}
