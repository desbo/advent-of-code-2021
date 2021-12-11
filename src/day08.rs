use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::Split;
use std::os::raw::c_char;
use std::result::IntoIter;
use std::str::{Chars, SplitWhitespace};

fn numbers_with_segment_count(segment_count: u8) -> Vec<u8> {
    match segment_count {
        0 | 1 => vec![],
        2 => vec![1],
        3 => vec![7],
        4 => vec![4],
        5 => vec![2, 3, 5],
        6 => vec![0, 6, 9],
        7 => vec![8],
        _ => vec![],
    }
}

pub fn part1(input: &[String]) -> u16 {
    let output_values = input
        .into_iter()
        .map(|reading| reading.split(" | ").collect::<Vec<&str>>()[1])
        .flat_map(|digits| digits.split_whitespace());

    output_values
        .filter(|digit_str| numbers_with_segment_count(digit_str.len() as u8).len() == 1)
        .count() as u16
}

fn update_mapping(digits: &Vec<&str>) {
    let mut signals_by_number: HashMap<u8, HashSet<&str>> = HashMap::new();

    digits.into_iter().for_each(|&digit_str| {
        numbers_with_segment_count(digit_str.len() as u8)
            .iter()
            .for_each(|&possible_number| {
                signals_by_number
                    .entry(possible_number)
                    .or_insert(HashSet::new())
                    .insert(digit_str);
            })
    });

    let chars_4: HashSet<char> = HashSet::from_iter(
        signals_by_number
            .get(&4)
            .unwrap()
            .into_iter()
            .map(|d| *d)
            .collect::<Vec<&str>>()[0]
            .chars(),
    );

    let (two, threeAndFive) = signals_by_number
        .into_values()
        .filter(|c| c.iter().collect::<Vec<&&str>>()[0].len() == 5)
        .map(|i| i.into_iter().collect::<Vec<&str>>())
        .partition(|sig| {
            sig.into_iter()
                .find(|a| {
                    HashSet::from_iter(a.chars())
                        .difference(&chars_4)
                        .collect::<Vec<&char>>()
                        .len()
                        == 3
                })
                .is_some()
        });

    println!("{:?}", two);
}

fn parse_input(input: &[String]) -> Vec<Vec<&str>> {
    input
        .into_iter()
        .map(|reading| reading.split(" | "))
        .flat_map(|digits| digits.map(|d| d.split_whitespace().collect::<Vec<&str>>()))
        .collect::<Vec<Vec<&str>>>()
}

pub fn part2(input: &[String]) -> u16 {
    // map correct segments (according to diagram in puzzle description) with randomised segments
    // initially, we don't know anything...
    let mut segment_mapping: HashMap<char, Option<char>> = HashMap::new();

    if let [input_values, output_values] = parse_input(input).as_slice() {
        update_mapping(input_values);
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn inspect() {
        drop(part2(&[String::from(EXAMPLE)]))
    }
}
