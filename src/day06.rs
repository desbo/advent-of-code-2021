use std::num::ParseIntError;
use std::str::FromStr;

pub struct School([u64; 9]);

impl School {
    fn step(&mut self) {
        for i in 0..self.0.len() {
            if i > 0 {
                self.0.swap(i - 1, i)
            }
        }

        self.0[6] += self.0[8];
    }

    fn n_steps(&mut self, n: u32) {
        (0..n + 1).for_each(|i| self.step())
    }

    fn population(&self) -> u64 {
        self.0.into_iter().fold(0, |a, b| a + b as u64)
    }
}

impl FromStr for School {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fish_by_age = [0u64; 9];

        s.split(",")
            .map(|n| u64::from_str(n).unwrap())
            .for_each(|fish| fish_by_age[fish as usize + 1] += 1);

        Ok(School(fish_by_age))
    }
}

pub fn part1(input: &str) -> u64 {
    let mut school = School::from_str(input).unwrap();
    school.n_steps(80);
    school.population()
}

pub fn part2(input: &str) -> u64 {
    let mut school = School::from_str(input).unwrap();
    school.n_steps(256);
    school.population()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut school80 = School::from_str("3,4,3,1,2").unwrap();
        let mut school256 = School::from_str("3,4,3,1,2").unwrap();

        school80.n_steps(80);
        school256.n_steps(256);

        assert_eq!(school80.population(), 5934);
        assert_eq!(school256.population(), 26984457539);
    }
}
