use itertools::Itertools;

use crate::harness::Harness;

pub fn part_1(harness: &Harness) -> String {
    harness
        .input()
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|&(index, _)| index != line.len() - 1)
                .map(|(first_digit_index, first_digit)| {
                    line.chars()
                        .skip(first_digit_index + 1)
                        .map(|second_digit| {
                            first_digit.to_digit(10).expect("invalid tens digit") as u64 * 10
                                + second_digit.to_digit(10).expect("invalid tens digit") as u64
                        })
                        .max()
                        .expect("iterator should have at least one element")
                })
                .max()
                .expect("iterator should have at least one element")
        })
        .sum::<u64>()
        .to_string()
}

pub fn part_2(harness: &Harness) -> String {
    fn max_joltage(input: &str, len: u32) -> u64 {
        let n = input.len();
        // dp[i][l] = max joltage using first i digits at length l
        let mut dp = vec![vec![0u64; (len + 1) as usize]; n];

        for i in 0..n {
            dp[i][1] = input
                .chars()
                .nth(i)
                .expect("invalid tens digit")
                .to_digit(10)
                .expect("invalid tens digit") as u64;
        }

        let mut curr = 0u64;
        for length in 2..=len {
            for i in 0..n {
                for j in 0..i {
                    dp[i][length as usize] = std::cmp::max(
                        dp[j][(length - 1) as usize] * 10
                            + input
                                .chars()
                                .nth(i)
                                .expect("invalid tens digit")
                                .to_digit(10)
                                .expect("invalid tens digit") as u64,
                        dp[i][length as usize],
                    );
                }
                curr = std::cmp::max(dp[i][length as usize], curr);
            }
        }

        curr
    }

    harness
        .input()
        .lines()
        .map(|line| max_joltage(line, 12))
        .inspect(|v| tracing::debug!("max joltage for line: {}", v))
        .sum::<u64>()
        .to_string()
}
