#![warn(clippy::pedantic, clippy::nursery)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::NonZeroU16;
use std::str::FromStr;
use std::time::Instant;

pub mod utils;

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
    #[allow(clippy::missing_errors_doc)]
    pub fn parse(text: &str) -> Result<Self, <usize as FromStr>::Err> {
        match text {
            "*" => Ok(Self::All),
            "." => Ok(Self::Latest),
            text => Ok(Self::Single(text.parse()?)),
        }
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn run_solutions(
    all_solutions: Vec<Vec<Vec<Box<dyn Solve>>>>,
    year_selection: Selection,
    day_selection: Selection,
    loop_count: Option<NonZeroU16>,
) {
    let (year_n, years) = match year_selection {
        Selection::All => (FIRST_YEAR, all_solutions),
        Selection::Latest => (
            all_solutions.len() + FIRST_YEAR - 1,
            vec![all_solutions.into_iter().last().unwrap()],
        ),
        Selection::Single(year_n) => {
            if year_n < FIRST_YEAR || year_n - FIRST_YEAR >= all_solutions.len() {
                eprintln!("no solutions available for year {year_n}");
                return;
            }

            (
                year_n,
                vec![all_solutions.into_iter().nth(year_n - FIRST_YEAR).unwrap()],
            )
        }
    };

    print_selection(year_selection, day_selection, year_n);

    for (year_offset, year) in years.into_iter().enumerate() {
        let (day_n, days) = match day_selection {
            Selection::All => (1, year),
            Selection::Latest => (year.len(), vec![year.into_iter().last().unwrap()]),
            Selection::Single(day_n) => {
                if day_n < 1 || day_n > year.len() {
                    eprintln!("no solution available for day {day_n} of year {year_n}");
                    continue;
                }

                (day_n, vec![year.into_iter().nth(day_n - 1).unwrap()])
            }
        };

        for (day_offset, day) in days
            .into_iter()
            .enumerate()
            .filter(|(_, day)| !day.is_empty())
        {
            run_solution(day, year_n + year_offset, day_n + day_offset, loop_count);
        }
    }
}

fn run_solution(
    parts: Vec<Box<dyn Solve>>,
    year: usize,
    day: usize,
    loop_count: Option<NonZeroU16>,
) {
    let input = BufReader::new(
        File::open(format!("src/year_{year}/day_{day}/input.txt"))
            .unwrap_or_else(|_| panic!("input file for year_{year}/day_{day} not found")),
    )
    .lines()
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

    for (part, part_n) in parts.into_iter().zip(1..) {
        let input_cloned = input.clone();

        let output = if let Some(loop_count) = loop_count {
            benchmark_part(part.as_ref(), &input_cloned, loop_count.get())
        } else {
            let start = Instant::now();
            let result = part.solve(input_cloned);
            let duration = start.elapsed();
            let check = if result == part.correct_solution() {
                "\x1B[32m✔\x1B[0m"
            } else {
                "\x1B[31m✘\x1B[0m"
            };
            format!("{check} {result} ({duration:?})")
        };

        println!("year {year}, day {day}, part {part_n}: {output}");
    }
}

fn benchmark_part(part: &dyn Solve, input: &[String], loop_count: u16) -> String {
    let mut timings = Vec::with_capacity(loop_count.into());

    for _ in 0..loop_count {
        let input_cloned = input.to_owned();
        let start = Instant::now();
        _ = part.solve(input_cloned);
        timings.push(start.elapsed());
    }

    timings.sort_unstable();
    let p5 = timings[usize::from(loop_count / 20)];
    format!("{loop_count} loops, top 5%: {p5:?} per loop")
}

fn print_selection(year_selection: Selection, day_selection: Selection, latest_year: usize) {
    let year_text = match year_selection {
        Selection::All => "all years".into(),
        Selection::Latest => format!("latest year ({latest_year})"),
        Selection::Single(value) => format!("year {value}"),
    };

    let day_text = match day_selection {
        Selection::All => "all days".into(),
        Selection::Latest => "latest day".into(),
        Selection::Single(value) => format!("day {value}"),
    };

    eprintln!("solving {day_text} of {year_text}");
}
