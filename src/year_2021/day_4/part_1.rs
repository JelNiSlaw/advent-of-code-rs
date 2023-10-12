use advent_of_code::Solve;

use super::shared::{parse_input, Board};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "34506"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let (winning_numbers, mut boards) = parse_input(lines);

        let (number, board) = run_bingo(&winning_numbers, &mut boards);

        (board.score() * u32::from(number)).to_string()
    }
}

fn run_bingo(winning_numbers: &[u8], boards: &mut [Board]) -> (u8, Board) {
    for &number in winning_numbers {
        for board in boards.iter_mut() {
            if board.mark(number) && board.is_winning() {
                return (number, *board);
            }
        }
    }

    unreachable!()
}
