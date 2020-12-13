mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::path::PathBuf;

const INPUT_DIR: &str = "input";

fn input_dir() -> PathBuf {
    use std::env;

    env::args()
        .skip(1)
        .next()
        .unwrap_or(INPUT_DIR.to_string())
        .into()
}

fn day<F>(day: i32, parts: F)
where
    F: Fn(&str),
{
    use std::fs;

    println!("Day: {}", day);

    let mut path = input_dir();
    path.push(format!("day{}", day));

    fs::read_to_string(&path)
        .map(|input| parts(&input))
        .unwrap_or_else(|err| {
            println!("Error opening input file {}: {}", path.to_str().unwrap(), err);
        });
}

fn part<F, O, E>(part: i32, f: F, input: &str)
where
    F: Fn(&str) -> Result<O, E>,
    O: std::fmt::Display,
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    println!("  Part {}:", part);

    match f(input) {
        Ok(output) => println!("    Output: {}", output),
        Err(err) => println!("    Error: {}", err.into()),
    };
}

fn main() {
    day(1, |input| {
        part(1, day1::part1, input);
        part(2, day1::part2, input);
    });

    day(2, |input| {
        part(1, day2::part1, input);
        part(2, day2::part2, input);
    });

    day(3, |input| {
        part(1, day3::part1, input);
        part(2, day3::part2, input);
    });

    day(4, |input| {
        part(1, day4::part1, input);
        part(2, day4::part2, input);
    });

    day(5, |input| {
        part(1, day5::part1, input);
        part(2, day5::part2, input);
    });

    day(6, |input| {
        part(1, day6::part1, input);
        part(2, day6::part2, input);
    });

    day(7, |input| {
        part(1, day7::part1, input);
        part(2, day7::part2, input);
    });

    day(8, |input| {
        part(1, day8::part1, input);
        part(2, day8::part2, input);
    });
}
