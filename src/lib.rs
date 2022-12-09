use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const FIRST_YEAR: usize = 2021;

pub trait Solve {
    fn correct_solution(&self) -> &str;
    fn solve(&self, lines: Vec<String>) -> String;
}

#[derive(Clone, Copy)]
pub enum Selection {
    All,
    Latest,
    Single(usize),
}

impl Selection {
    pub fn parse(text: &str) -> Result<Self, <usize as FromStr>::Err> {
        match text {
            "*" => Ok(Self::All),
            "." => Ok(Self::Latest),
            text => Ok(Selection::Single(text.parse()?)),
        }
    }
}

pub fn run_solutions(
    all_solutions: Vec<Vec<Vec<Box<dyn Solve>>>>,
    year_selection: Selection,
    day_selection: Selection,
) {
    let (year_n, years) = match year_selection {
        Selection::All => (FIRST_YEAR, all_solutions),
        Selection::Latest => (
            all_solutions.len() + FIRST_YEAR - 1,
            vec![all_solutions.into_iter().last().unwrap()],
        ),
        Selection::Single(year_n) => {
            if year_n < FIRST_YEAR || year_n - FIRST_YEAR >= all_solutions.len() {
                println!("no solutions available for year {year_n}");
                return;
            }

            (
                year_n,
                vec![all_solutions.into_iter().nth(year_n - FIRST_YEAR).unwrap()],
            )
        }
    };

    print_selection(year_selection, day_selection);

    for (year_offset, year) in years.into_iter().enumerate() {
        let (day_n, days) = match day_selection {
            Selection::All => (1, year),
            Selection::Latest => (year.len(), vec![year.into_iter().last().unwrap()]),
            Selection::Single(day_n) => {
                if day_n < 1 || day_n > year.len() {
                    println!("no solution available for day {day_n} of year {year_n}");
                    continue;
                }

                (day_n, vec![year.into_iter().nth(day_n - 1).unwrap()])
            }
        };

        for (day_offset, day) in days.into_iter().enumerate() {
            run_solution(day, year_n + year_offset, day_n + day_offset);
        }
    }
}

fn run_solution(parts: Vec<Box<dyn Solve>>, year: usize, day: usize) {
    let input = BufReader::new(
        File::open(format!("src/year_{year}/day_{day}/input.txt"))
            .unwrap_or_else(|_| panic!("input file for year_{year}/day_{day} not found")),
    )
    .lines()
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

    for (part_n, part) in parts.into_iter().enumerate() {
        let part_n = part_n + 1;
        let result = part.solve(input.clone());
        let check = if result == part.correct_solution() {
            '✅'
        } else {
            '❌'
        };
        println!("year {year}, day {day}, part {part_n}: {check} {result}");
    }
}

fn print_selection(year_selection: Selection, day_selection: Selection) {
    let year_text = match year_selection {
        Selection::All => "all years".into(),
        Selection::Latest => "latest year".into(),
        Selection::Single(value) => format!("year {value}"),
    };

    let day_text = match day_selection {
        Selection::All => "all days".into(),
        Selection::Latest => "latest day".into(),
        Selection::Single(value) => format!("day {value}"),
    };

    println!("solving {day_text} of {year_text}");
}
