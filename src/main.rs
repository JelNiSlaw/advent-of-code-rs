#![warn(clippy::pedantic, clippy::nursery)]

use std::env;
use std::num::NonZeroU16;

use advent_of_code::{Selection, Solve};

mod year_2021;
mod year_2022;
mod year_2023;

#[must_use]
pub fn years() -> Vec<Vec<Vec<Box<dyn Solve>>>> {
    vec![year_2021::days(), year_2022::days(), year_2023::days()]
}

fn main() {
    let (year_selection, day_selection, loop_count) = match parse_arguments(env::args().skip(1)) {
        Ok(arguments) => arguments,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    advent_of_code::run_solutions(years(), year_selection, day_selection, loop_count);
}

fn parse_arguments(
    mut args: impl Iterator<Item = String>,
) -> Result<(Selection, Selection, Option<NonZeroU16>), String> {
    let (year_selection, day_selection) = parse_days(args.by_ref())?;
    Ok((year_selection, day_selection, parse_loops(args)?))
}

fn parse_days(mut args: impl Iterator<Item = String>) -> Result<(Selection, Selection), String> {
    let Some(first_arg) = args.next() else {
        return Ok((Selection::Latest, Selection::Latest));
    };

    let Some(second_arg) = args.next() else {
        let first_selection =
            Selection::parse(&first_arg).map_err(|_| format!("invalid day {first_arg:?}"))?;

        return match first_selection {
            Selection::Single(value) => {
                if value >= 1000 {
                    Ok((first_selection, Selection::All))
                } else {
                    Ok((Selection::Latest, first_selection))
                }
            }
            _ => Ok((Selection::Latest, first_selection)),
        };
    };

    let first_selection =
        Selection::parse(&first_arg).map_err(|_| format!("invalid year {first_arg:?}"))?;

    let second_selection =
        Selection::parse(&second_arg).map_err(|_| format!("invalid day {second_arg:?}"))?;

    Ok((first_selection, second_selection))
}

fn parse_loops(mut args: impl Iterator<Item = String>) -> Result<Option<NonZeroU16>, String> {
    let Some(third_argument) = args.next() else {
        return Ok(None);
    };

    let loop_count: u16 = third_argument
        .parse()
        .map_err(|_| format!("invalid loop count {third_argument:?}"))?;

    Ok(NonZeroU16::new(loop_count))
}
