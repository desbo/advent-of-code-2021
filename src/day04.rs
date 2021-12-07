use indexmap::IndexMap;
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
                    line.split_whitespace()
                        .map(|num| (u8::from_str(num).unwrap(), false))
                })
                .collect(),
        })
    }
}

impl Card {
    fn mark(&mut self, number: u8) {
        self.numbers.insert(number, true);
    }

    fn marked_at_index(&self, i: usize) -> bool {
        self.numbers
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
            let bingo =
                (0..self.width).all(|i| self.marked_at_index(((i * self.width) + col).into()));

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
        self.numbers
            .iter()
            .filter_map(|(n, marked)| if *marked { None } else { Some(*n) })
    }
}

fn play_game(drawn_numbers: &[u8], cards: &mut [Card]) -> Option<(&Card, u8)> {
    drawn_numbers.iter().find_map(|&n| {
        cards.iter_mut().for_each(|c| c.mark(n));
        cards.iter().find(|c| c.bingo()).map(|c| (c, n))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
