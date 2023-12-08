use advent_of_code::Solve;

use super::shared::Map;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "16563603485021"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let map = Map::parse(lines);

        map.nodes()
            .filter(|node| node[2] == b'A')
            .map(|&node| map.path_length(node, |node| node[2] == b'Z'))
            .reduce(least_common_multiple)
            .unwrap()
            .to_string()
    }
}

const fn least_common_multiple(a: u64, b: u64) -> u64 {
    a * b / greatest_common_divisor(a, b)
}

const fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}
