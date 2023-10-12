use advent_of_code::Solve;

use super::shared::{parse_input, Board};

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "7686"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let (winning_numbers, boards) = parse_input(lines);

        let (number, board) = find_last_board(&winning_numbers, boards);

        (board.score() * u32::from(number)).to_string()
    }
}

fn find_last_board(winning_numbers: &[u8], mut boards: Vec<Board>) -> (u8, Board) {
    let mut winning_numbers = winning_numbers.iter();

    for &number in winning_numbers.by_ref() {
        for board in &mut boards {
            board.mark(number);
        }

        boards.retain(|board| !board.is_winning());

        if boards.len() == 1 {
            break;
        }
    }

    let mut board = boards.into_iter().next().unwrap();

    for &number in winning_numbers {
        board.mark(number);

        if board.is_winning() {
            return (number, board);
        }
    }

    unreachable!()
}
