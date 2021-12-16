use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::{Hash, Hasher},
    str::FromStr,
};

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

struct Signal {
    chars: String,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.chars)
    }
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signal {
            chars: String::from(s),
        })
    }
}

impl Hash for Signal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut chars_copy = self.chars.clone().chars().collect::<Vec<char>>();
        chars_copy.sort();
        chars_copy.hash(state)
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        let mut chars_copy = self.chars.clone().chars().collect::<Vec<char>>();
        let mut other_copy = other.chars.clone().chars().collect::<Vec<char>>();
        chars_copy.sort();
        other_copy.sort();
        chars_copy == other_copy
    }
}
impl Eq for Signal {}

impl Clone for Signal {
    fn clone(&self) -> Self {
        Self {
            chars: self.chars.clone(),
        }
    }
}

impl Signal {
    fn len(&self) -> usize {
        self.chars.len()
    }
}

fn digits_by_signal(digits: &Vec<Signal>) -> HashMap<Signal, u8> {
    let mut candidate_signals_by_number: HashMap<u8, HashSet<Signal>> = HashMap::new();
    let mut known_signals_by_number: HashMap<u8, Signal> = HashMap::new();

    digits.into_iter().clone().for_each(|signal| {
        let possible_numbers = numbers_with_segment_count(signal.len() as u8);

        if possible_numbers.len() == 1 {
            known_signals_by_number.insert(possible_numbers[0], signal.clone());
        } else {
            possible_numbers
                .iter()
                .clone()
                .for_each(|&possible_number| {
                    candidate_signals_by_number
                        .entry(possible_number)
                        .or_insert(HashSet::new())
                        .insert(signal.clone());
                })
        }
    });

    let signal_4: &Signal = known_signals_by_number.get(&4).unwrap();

    // [2,3,5]
    let five_character_signals = candidate_signals_by_number
        .clone()
        .into_values()
        .filter(|c| c.iter().collect::<Vec<&Signal>>()[0].len() == 5)
        .flatten();

    let (two, three_and_five): (HashSet<Signal>, HashSet<Signal>) = five_character_signals
        .partition(|sig| {
            let sig_chars: HashSet<char> = HashSet::from_iter(sig.chars.chars());

            sig_chars
                .difference(&HashSet::from_iter(signal_4.chars.chars()))
                .collect::<Vec<&char>>()
                .len()
                == 3
        });

    known_signals_by_number.insert(2, two.into_iter().collect::<Vec<Signal>>()[0].clone());

    let chars_2: HashSet<char> =
        HashSet::from_iter(known_signals_by_number.get(&2).unwrap().chars.chars());

    if let [three_or_five, five_or_three] = three_and_five
        .clone()
        .into_iter()
        .map(|c| HashSet::from_iter(c.chars.chars()))
        .collect::<Vec<HashSet<char>>>()
        .as_slice()
    {
        let b_and_c = three_or_five.symmetric_difference(five_or_three);
        let (c, _b): (Vec<char>, Vec<char>) = b_and_c.partition(|chr| chars_2.contains(chr));
        let (three, five): (Vec<Signal>, Vec<Signal>) = three_and_five
            .into_iter()
            .partition(|str| str.chars.contains(c[0]));

        known_signals_by_number.insert(3, three[0].clone());
        known_signals_by_number.insert(5, five[0].clone());
    }

    // [0,6,9]
    let six_character_signals = candidate_signals_by_number
        .clone()
        .into_values()
        .filter(|c| c.iter().collect::<Vec<&Signal>>()[0].len() == 6)
        .flatten();

    let chars_4: HashSet<char> =
        HashSet::from_iter(known_signals_by_number.get(&4).unwrap().chars.chars());

    let (nine, zero_and_six): (Vec<Signal>, Vec<Signal>) = six_character_signals.partition(|str| {
        chars_4
            .difference(&HashSet::from_iter(str.chars.chars()))
            .collect::<Vec<&char>>()
            .len()
            == 0
    });

    println!("{:?}, {:?}", zero_and_six, nine);

    known_signals_by_number.insert(9, nine[0].clone());

    let chars_1: HashSet<char> =
        HashSet::from_iter(known_signals_by_number.get(&1).unwrap().chars.chars());

    let (zero, six): (Vec<Signal>, Vec<Signal>) = zero_and_six
        .into_iter()
        .partition(|signal| chars_1.iter().all(|c| signal.chars.contains(*c)));

    println!("{:?}, {:?}", zero, six);

    known_signals_by_number.insert(0, zero[0].clone());
    known_signals_by_number.insert(6, six[0].clone());

    HashMap::from_iter(
        known_signals_by_number
            .into_iter()
            .map(|(digit, signal)| (signal, digit)),
    )
}

fn parse_input(lines: &[String]) -> impl Iterator<Item = (Vec<Signal>, Vec<Signal>)> + '_ {
    lines.iter().map(|line| {
        if let [input, output] = line.split(" | ").collect::<Vec<&str>>().as_slice() {
            (
                input
                    .split_whitespace()
                    .map(|s| Signal::from_str(s).unwrap())
                    .collect::<Vec<Signal>>(),
                output
                    .split_whitespace()
                    .map(|s| Signal::from_str(s).unwrap())
                    .collect::<Vec<Signal>>(),
            )
        } else {
            panic!()
        }
    })
}

pub fn part2(input: &[String]) -> u32 {
    parse_input(input).fold(0, |a, line| {
        let (input_values, output_values) = line;
        let joined = [input_values.clone(), output_values.clone()].concat();
        let lookup = digits_by_signal(&joined);

        println!("{:?}", lookup);

        let output_str = &output_values
            .iter()
            .map(|s| lookup.get(s).unwrap().to_string())
            .collect::<Vec<String>>()
            .join("");

        println!("{}", output_str);

        a + u32::from_str(output_str).unwrap()
    })
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
