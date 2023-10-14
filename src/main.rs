#![warn(clippy::pedantic, clippy::nursery)]

use std::env;

use advent_of_code::{Selection, Solve};

mod year_2021;
mod year_2022;

#[must_use]
pub fn years() -> Vec<Vec<Vec<Box<dyn Solve>>>> {
    vec![year_2021::days(), year_2022::days()]
}

fn main() {
    let mut args = env::args().skip(1);

    let (year_selection, day_selection) = match parse_arguments(args.next(), args.next()) {
        Ok(arguments) => arguments,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    advent_of_code::run_solutions(years(), year_selection, day_selection);
}

fn parse_arguments(
    first_arg: Option<String>,
    second_arg: Option<String>,
) -> Result<(Selection, Selection), String> {
    let Some(first_arg) = first_arg else {
        return Ok((Selection::Latest, Selection::Latest));
    };

    let Some(second_arg) = second_arg else {
        let Ok(first_selection) = Selection::parse(&first_arg) else {
            return Err(format!("invalid day {first_arg:?}"));
        };

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

    let Ok(first_selection) = Selection::parse(&first_arg) else {
        return Err(format!("invalid year {first_arg:?}"));
    };

    let Ok(second_selection) = Selection::parse(&second_arg) else {
        return Err(format!("invalid day {second_arg:?}"));
    };

    Ok((first_selection, second_selection))
}
