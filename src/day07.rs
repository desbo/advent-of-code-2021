use std::str::FromStr;

fn median(nums: &mut [u64]) -> u64 {
    nums.sort();

    if nums.len() % 2 == 0 {
        nums[nums.len() / 2]
    } else {
        (nums[(nums.len() / 2)] + nums[(nums.len() / 2) + 1]) / 2
    }
}

pub fn part1(input: &str) -> u64 {
    let mut nums = input
        .split(",")
        .map(|n| u64::from_str(n).unwrap())
        .collect::<Vec<u64>>();

    let med = median(nums.as_mut());

    nums.into_iter().fold(0, |fuel, position| {
        fuel + i64::abs(position as i64 - med as i64) as u64
    })
}

pub fn part2(input: &str) -> u64 {
    let mut nums = input
        .split(",")
        .map(|n| u64::from_str(n).unwrap())
        .collect::<Vec<u64>>();

    let usage = |d| (d * (d + 1)) / 2;
    let mut min_usage = f64::INFINITY;

    nums.sort();

    for &i in &nums {
        let mut total_usage: u64 = 0;
        for &j in &nums {
            total_usage += usage(i64::abs(j as i64 - i as i64) as u64);
        }
        min_usage = f64::min(min_usage, total_usage as f64)
    }

    min_usage as u64
}
