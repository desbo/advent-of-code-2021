use std::num::ParseIntError;
use std::str::FromStr;

pub struct School(Vec<u8>);

impl School {
    fn step(&mut self) {
        for i in 0..self.0.len() {
            match self.0[i] {
                0 => {
                    self.0[i] = 6;
                    self.0.push(8);
                }
                _ => self.0[i] -= 1,
            }
        }
    }

    fn n_steps(&mut self, n: u32) {
        (0..n).for_each(|i| {
            self.step();
            println!("population after day {}: \t{}", i + 1, self.0.len());
        })
    }
}

impl FromStr for School {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(",")
            .map(|n| u8::from_str(n))
            .collect::<Result<Vec<u8>, ParseIntError>>()
            .map(|fish| School(fish))
    }
}

pub fn part1(input: &str) -> usize {
    let mut school = School::from_str(input).unwrap();
    school.n_steps(80);
    school.0.len()
}

pub fn part2(input: &str) -> usize {
    let mut school = School::from_str(input).unwrap();
    school.n_steps(256);
    school.0.len()
}
