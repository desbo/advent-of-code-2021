use std::ops::Add;

pub fn solve(report: &[String]) -> u32 {
    let binary_report = report.iter().map(|s| binary_string_to_bits(s).collect());

    let counts = count_bits(binary_report);

    let mut gamma_bits = Vec::new();
    let mut epsilon_bits = Vec::new();

    for i in 0..counts.len() {
        let common = counts[i] > report.len() as u32 / 2;

        gamma_bits.push(u8::from(common));
        epsilon_bits.push(u8::from(!common));
    }

    u32_from_bits(gamma_bits) * u32_from_bits(epsilon_bits)
}

fn count_bits(slices: impl Iterator<Item = Vec<u32>>) -> Vec<u32> {
    slices
        .reduce(|counts, reading| counts.iter().zip(reading).map(|(a, b)| a + b).collect())
        .unwrap()
}

fn binary_string_to_bits(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.chars().map(|c| if c == '1' { 1 } else { 0 })
}

fn u32_from_bits(bits: Vec<u8>) -> u32 {
    let s = bits
        .iter()
        .map(|b| b.to_string())
        .reduce(|x, x1| x.add(&x1))
        .unwrap();

    u32::from_str_radix(&s, 2).unwrap()
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
            count_bits(
                [
                    vec![0, 0, 1],
                    vec![0, 1, 0],
                    vec![1, 0, 0],
                    vec![1, 1, 0],
                    vec![1, 0, 0]
                ]
                .into_iter()
            ),
            [3, 2, 1]
        )
    }

    #[test]
    fn example() {
        let data: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect();

        assert_eq!(solve(&data), 198)
    }
}
