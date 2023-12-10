use advent_of_code::Solve;

use super::shared::Maze;

pub struct Solution;

impl Solve for Solution {
    fn correct_solution(&self) -> &str {
        "291"
    }

    fn solve(&self, lines: Vec<String>) -> String {
        let maze = Maze::parse(lines);

        let mut area = 0;

        for row in maze.pipe_loop_locations() {
            let mut inside = false;
            for pipe in row {
                if let Some(pipe) = pipe {
                    if pipe.is_north_facing() {
                        inside = !inside;
                    }
                } else if inside {
                    area += 1;
                }
            }
        }

        area.to_string()
    }
}
