use std::env;
use std::fmt::Debug;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

mod day01;
mod day02;

fn main() {
    match env::args().nth(1).unwrap().as_str() {
        "1a" => println!("{}", day01::part1(read_lines_stdin())),
        "1b" => println!("{}", day01::part2(read_lines_stdin())),
        "2" => println!("{}", day02::solve(read_lines_stdin())),
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
