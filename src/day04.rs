use indexmap::IndexMap;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
struct Card {
    width: u8,
    numbers: IndexMap<u8, bool>,
}

impl Card {
    fn from_grid_string(grid: &str) -> Card {
        let lines = grid.trim().lines();
        let width = grid
            .trim()
            .lines()
            .collect::<Vec<&str>>()
            .first()
            .map_or(0, |x| {
                x.split_whitespace().collect::<Vec<&str>>().len() as u8
            });

        Card {
            width,
            numbers: lines
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(|num| (u8::from_str(num).unwrap(), false))
                })
                .collect(),
        }
    }

    fn mark(&mut self, number: u8) {
        self.numbers.insert(number, true);
    }

    fn horizontal_bingo(&self) -> bool {
        let mut row = 0;

        while row < (self.numbers.len() as u8 - self.width) {
            let bingo = (row..row + self.width).all(|i| {
                self.numbers
                    .get_index(i.into())
                    .map_or(false, |(_n, marked)| *marked)
            });

            if bingo {
                return true;
            } else {
                row += self.width
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_creation() {
        let card = Card::from_grid_string(
            "1 2 3
            4 5 6
            7 8 9",
        );

        let mut expected_squares: IndexMap<u8, bool> = IndexMap::new();

        (1..10).for_each(|x| drop(expected_squares.insert(x, false)));

        assert_eq!(
            card,
            Card {
                width: 3,
                numbers: expected_squares
            }
        )
    }

    #[test]
    fn horizontal_bingo() {
        let mut card = Card::from_grid_string(
            "1 2 3
            4 5 6
            7 8 9",
        );

        assert!(!card.horizontal_bingo());

        vec![4, 5, 6].iter().for_each(|x: &u8| card.mark(*x));

        assert!(card.horizontal_bingo());
    }
}
