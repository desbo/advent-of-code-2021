use indexmap::IndexMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Card {
  width: u8,
  numbers: IndexMap<u8, bool>,
}

impl FromStr for Card {
  type Err = ();

  fn from_str(grid: &str) -> Result<Self, Self::Err> {
    let lines = grid.trim().lines();
    let width = grid
      .trim()
      .lines()
      .collect::<Vec<&str>>()
      .first()
      .map_or(0, |x| {
        x.split_whitespace().collect::<Vec<&str>>().len() as u8
      });

    Ok(Card {
      width,
      numbers: lines
        .flat_map(|line| {
          line
            .split_whitespace()
            .map(|num| (u8::from_str(num).unwrap(), false))
        })
        .collect(),
    })
  }
}

impl Card {
  fn mark(&mut self, number: u8) {
    if self.numbers.contains_key(&number) {
      self.numbers.insert(number, true);
    }
  }

  fn marked_at_index(&self, i: usize) -> bool {
    self
      .numbers
      .get_index(i.into())
      .map_or(false, |(_n, marked)| *marked)
  }

  fn horizontal_bingo(&self) -> bool {
    let mut row = 0;

    while row < (self.numbers.len() as u8 - self.width) {
      let bingo = (row..row + self.width).all(|i| self.marked_at_index(i.into()));

      if bingo {
        return true;
      } else {
        row += self.width;
      }
    }

    false
  }

  fn vertical_bingo(&self) -> bool {
    let mut col = 0;

    while col < self.width {
      // assumption: bingo cards are always square, so we can use width
      // for the vertical offset
      let bingo = (0..self.width).all(|i| self.marked_at_index(((i * self.width) + col).into()));

      if bingo {
        return true;
      } else {
        col += 1;
      }
    }

    false
  }

  fn bingo(&self) -> bool {
    self.horizontal_bingo() || self.vertical_bingo()
  }

  fn unmarked(&self) -> impl Iterator<Item = u8> + '_ {
    self
      .numbers
      .iter()
      .filter_map(|(n, marked)| if *marked { None } else { Some(*n) })
  }
}

fn parse_input(input: &[String]) -> (Vec<u8>, Vec<Card>) {
  let numbers: Vec<u8> = input
    .iter()
    .take(1)
    .flat_map(|nums| {
      nums
        .split(",")
        .map(|x| u8::from_str(x).unwrap())
        .collect::<Vec<u8>>()
    })
    .collect();

  let cards: Vec<Card> = input
    .split_at(2)
    .1
    .split(|s| s.trim() == "")
    .map(|s| Card::from_str(&*s.join("\n")).unwrap())
    .collect();

  (numbers, cards)
}

pub fn part1(input: &[String]) -> u32 {
  let (numbers, mut cards) = parse_input(input);

  for number in numbers {
    for card in cards.iter_mut() {
      card.mark(number);

      if card.bingo() {
        let result: u32 = card.unmarked().map(|x| x as u32).sum();
        return number as u32 * result;
      }
    }
  }

  return 0;
}

pub fn part2(input: &[String]) -> u32 {
  let (numbers, mut cards) = parse_input(input);
  let card_count = cards.len();
  let mut winning_card_indexes = HashSet::new();

  for number in numbers {
    for (i, card) in cards.iter_mut().enumerate() {
      card.mark(number);

      if card.bingo() && !winning_card_indexes.contains(&i) {
        winning_card_indexes.insert(i);

        if winning_card_indexes.len() == card_count {
          let result: u32 = card.unmarked().map(|x| x as u32).sum();
          return number as u32 * result;
        }
      }
    }
  }

  return 0;
}
#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

  #[test]
  fn example_part1() {
    assert_eq!(
      part1(&EXAMPLE.lines().map(String::from).collect::<Vec<String>>()),
      4512
    )
  }

  #[test]
  fn example_part2() {
    assert_eq!(
      part2(&EXAMPLE.lines().map(String::from).collect::<Vec<String>>()),
      1924
    )
  }

  #[test]
  fn card_creation() {
    let card = Card::from_str(
      "1 2 3
            4 5 6
            7 8 9",
    );

    let mut expected_squares: IndexMap<u8, bool> = IndexMap::new();

    (1..10).for_each(|x| drop(expected_squares.insert(x, false)));

    assert_eq!(
      card.unwrap(),
      Card {
        width: 3,
        numbers: expected_squares
      }
    )
  }

  #[test]
  fn horizontal_bingo() {
    let mut card = Card::from_str(
      "1 2 3
            4 5 6
            7 8 9",
    )
    .unwrap();

    assert!(!card.horizontal_bingo());
    vec![4, 5, 6].iter().for_each(|x: &u8| card.mark(*x));
    assert!(card.horizontal_bingo());
  }

  #[test]
  fn vertical_bingo() {
    let mut card = Card::from_str(
      "1 2 3
            4 5 6
            7 8 9",
    )
    .unwrap();

    assert!(!card.vertical_bingo());
    vec![1, 4, 7].iter().for_each(|x: &u8| card.mark(*x));
    assert!(card.vertical_bingo());
  }

  #[test]
  fn unmarked() {
    let mut card = Card::from_str(
      "1 2 3
            4 5 6
            7 8 9",
    )
    .unwrap();

    vec![1, 2, 3, 4].iter().for_each(|x: &u8| card.mark(*x));
    assert_eq!(card.unmarked().collect::<Vec<u8>>(), vec![5, 6, 7, 8, 9])
  }
}
