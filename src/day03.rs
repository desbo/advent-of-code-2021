pub fn part1(report: &[String]) -> u32 {
    let counts = count_bits(report);

    let mut gamma_bits = Vec::new();
    let mut epsilon_bits = Vec::new();

    for i in 0..counts.len() {
        let common = counts[i] > report.len() as u32 / 2;

        gamma_bits.push(u8::from(common));
        epsilon_bits.push(u8::from(!common));
    }

    u32_from_bits(gamma_bits) * u32_from_bits(epsilon_bits)
}

pub fn part2(report: &[String]) -> u32 {
    #[derive(PartialEq)]
    enum SearchMode {
        MostCommon,
        LeastCommon,
    }

    fn search(report: &[String], pos: usize, mode: SearchMode) -> String {
        let counts = count_bits(report);

        let common = counts[pos] * 2 >= report.len() as u32;

        let search_bit = match mode {
            SearchMode::MostCommon => common,
            SearchMode::LeastCommon => !common,
        };

        let search_bit_char = if search_bit { '1' } else { '0' };

        let filtered_report: Vec<String> = report
            .iter()
            .filter(|bit_string| bit_string.chars().nth(pos).unwrap() == search_bit_char)
            .cloned()
            .collect();

        if filtered_report.len() == 1 {
            filtered_report[0].clone()
        } else {
            search(&filtered_report, pos + 1, mode)
        }
    }

    let oxy = search(report, 0, SearchMode::MostCommon);
    let co2 = search(report, 0, SearchMode::LeastCommon);

    let to_int = |str| u32::from_str_radix(str, 2).unwrap();

    to_int(&oxy) * to_int(&co2)
}

fn count_bits(report: &[String]) -> Vec<u32> {
    report
        .into_iter()
        .map(|s| binary_string_to_bits(s).collect::<Vec<u32>>())
        .reduce(|counts, reading| counts.iter().zip(reading).map(|(a, b)| a + b).collect())
        .unwrap()
}

fn binary_string_to_bits(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.chars().map(|c| if c == '1' { 1 } else { 0 })
}

fn u32_from_bits(bits: Vec<u8>) -> u32 {
    u32::from_str_radix(&bits_to_binary_string(bits), 2).unwrap()
}

fn bits_to_binary_string(bits: Vec<u8>) -> String {
    bits.iter().map(|b| b.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_bits() {
        assert_eq!(
            binary_string_to_bits("00100").collect::<Vec<u32>>(),
            vec![0, 0, 1, 0, 0]
        );

        assert_eq!(
            binary_string_to_bits("11110").collect::<Vec<u32>>(),
            vec![1, 1, 1, 1, 0]
        );
    }

    #[test]
    fn test_count_bits() {
        assert_eq!(
            count_bits(&[
                "001".to_string(),
                "010".to_string(),
                "100".to_string(),
                "110".to_string(),
                "100".to_string()
            ]),
            [3, 2, 1]
        )
    }

    #[test]
    fn example_part1() {
        let data: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect();

        assert_eq!(part1(&data), 198)
    }

    #[test]
    fn example_part2() {
        let data: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect();

        assert_eq!(part2(&data), 230)
    }
}
