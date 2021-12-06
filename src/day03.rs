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
    let counts = count_bits(report);

    let mut most_common_bits = Vec::new();
    let mut least_common_bits = Vec::new();

    for i in 0..counts.len() {
        if counts[i] * 2 == report.len() as u32 {
            println!("HERE");
            most_common_bits.push(1);
            least_common_bits.push(0);
        } else {
            let common = counts[i] > report.len() as u32 / 2;
            most_common_bits.push(u8::from(common));
            least_common_bits.push(u8::from(!common));
        }
    }

    let mut most_common_search_string = bits_to_binary_string(most_common_bits);
    let mut least_common_search_string = bits_to_binary_string(least_common_bits);

    println!(
        "most common: {}\t least common: {}",
        most_common_search_string, least_common_search_string
    );

    let mut oxy = "";
    let mut co2 = "";

    let search = |search_string: &String| report.iter().find(|s| s.starts_with(search_string));

    loop {
        println!(
            "most common: {}\t least common: {}",
            most_common_search_string, least_common_search_string
        );

        if oxy == "" {
            search(&most_common_search_string).map_or_else(
                || drop(most_common_search_string.pop()),
                |result| oxy = result,
            )
        }

        if co2 == "" {
            search(&least_common_search_string).map_or_else(
                || drop(least_common_search_string.pop()),
                |result| co2 = result,
            )
        }

        println!("oxy: {}\t co2: {}", oxy, co2);

        if oxy != "" && co2 != "" {
            return u32::from_str_radix(&oxy, 2).unwrap() * u32::from_str_radix(&co2, 2).unwrap();
        }
    }
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
