#![warn(clippy::pedantic)]

use std::env;

use advent_of_code::{Selection, Solve};
mod year_2022;

#[must_use]
pub fn years() -> Vec<Vec<Vec<Box<dyn Solve>>>> {
    vec![year_2022::days()]
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
    match first_arg {
        Some(first_arg) => match second_arg {
            Some(second_arg) => Ok((
                match Selection::parse(&first_arg) {
                    Ok(first_selection) => first_selection,
                    Err(_) => {
                        return Err(format!("invalid day {first_arg:?}"));
                    }
                },
                match Selection::parse(&second_arg) {
                    Ok(first_selection) => first_selection,
                    Err(_) => {
                        return Err(format!("invalid year {second_arg:?}"));
                    }
                },
            )),
            None => match Selection::parse(&first_arg) {
                Ok(first_selection) => match first_selection {
                    Selection::All | Selection::Latest => Ok((Selection::Latest, first_selection)),
                    Selection::Single(value) => {
                        if value >= 1000 {
                            Ok((first_selection, Selection::All))
                        } else {
                            Ok((Selection::Latest, first_selection))
                        }
                    }
                },
                Err(_) => {
                    return Err(format!("invalid day {first_arg:?}"));
                }
            },
        },
        None => Ok((Selection::Latest, Selection::Latest)),
    }
}
