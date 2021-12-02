pub fn part1(report: Vec<i32>) -> i32 {
    report.windows(2).fold(0, |increases, pair| {
        if pair[1] > pair[0] {
            increases + 1
        } else {
            increases
        }
    })
}

pub fn part2(report: Vec<i32>) -> i32 {
    let sums: Vec<i32> = report
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect();

    sums.windows(2).fold(0, |increases, pair| {
        if pair[1] > pair[0] {
            increases + 1
        } else {
            increases
        }
    })
}
