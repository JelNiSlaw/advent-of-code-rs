use advent_of_code::Solve;

use super::shared::get_directions;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "392080"
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

        let mut scores = Vec::new();
        for row in 1..forest.len() - 1 {
            for collumn in 1..forest[row].len() - 1 {
                let score = scenic_score((row, collumn), &forest);
                scores.push(score);
            }
        }

        scores.into_iter().max().unwrap().to_string()
    }
}

fn scenic_score(location: (usize, usize), forest: &[Vec<u32>]) -> u32 {
    let tree_height = forest[location.0][location.1];

    let [mut left, right, mut up, down] = get_directions(location, forest);
    left.reverse();
    up.reverse();

    visible_trees(left, tree_height)
        * visible_trees(right, tree_height)
        * visible_trees(up, tree_height)
        * visible_trees(down, tree_height)
}

fn visible_trees(trees: Vec<u32>, max_height: u32) -> u32 {
    let mut score = 0;
    for height in trees {
        score += 1;
        if height >= max_height {
            break;
        }
    }

    score
}
