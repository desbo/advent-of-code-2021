use std::env;
use std::fmt::Debug;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    match env::args().nth(1).unwrap().as_str() {
        "1a" => println!("{}", day01::part1(&read_lines_stdin())),
        "1b" => println!("{}", day01::part2(&read_lines_stdin())),
        "2" => println!("{}", day02::solve(&read_lines_stdin())),
        "3a" => println!("{}", day03::part1(&read_lines_stdin())),
        "3b" => println!("{}", day03::part2(&read_lines_stdin())),
        "4a" => println!("{}", day04::part1(&read_lines_stdin())),
        "4b" => println!("{}", day04::part2(&read_lines_stdin())),
        "5a" => println!("{}", day05::part1(&read_lines_stdin())),
        "5b" => println!("{}", day05::part2(&read_lines_stdin())),
        "6a" => println!("{}", day06::part1(&read_lines_stdin::<String>().concat())),
        "6b" => println!("{}", day06::part2(&read_lines_stdin::<String>().concat())),
        _ => (),
    }
}

fn read_lines_stdin<T: FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .map(|result| result.unwrap().parse().unwrap())
        .collect()
}
