use std::collections::HashMap;

pub fn part1(input: &[String]) -> u16 {
    let numbers_by_segment_count: HashMap<u8, Vec<u8>> = HashMap::from([
        (0, vec![]),
        (1, vec![]),
        (2, vec![1]),
        (3, vec![7]),
        (4, vec![4]),
        (5, vec![2, 3, 5]),
        (6, vec![0, 6, 9]),
        (7, vec![8]),
    ]);g

    let output_values = input
        .into_iter()
        .map(|reading| reading.split(" | ").collect::<Vec<&str>>()[1])
        .flat_map(|digits| digits.split_whitespace());

    output_values
        .filter(|digit_signal| {
            numbers_by_segment_count
                .get(&(digit_signal.len() as u8))
                .map_or(false, |count| count.len() == 1)
        })
        .count() as u16
}
