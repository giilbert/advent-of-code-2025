use itertools::Itertools;

use crate::harness::Harness;

pub fn part_1(harness: &Harness) -> String {
    let lines = harness
        .input()
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.trim().to_string())
                .filter(|s| s != "")
                .collect_vec()
        })
        .collect_vec();

    let nums = &lines[0..lines.len() - 1];
    let last = &lines[lines.len() - 1];

    last.iter()
        .enumerate()
        .map(|(j, op)| {
            let mut total = if op == "*" { 1 } else { 0 };
            for i in 0..nums.len() {
                let n: u64 = nums[i][j].parse().unwrap();
                println!("nums[{}][{}] = {}", i, j, n);

                match op.as_str() {
                    "*" => total *= n,
                    "+" => total += n,
                    _ => panic!("unknown op"),
                }
            }
            println!("total for column {} = {}", j, total);
            total
        })
        .sum::<u64>()
        .to_string()
}

pub fn part_2(harness: &Harness) -> String {
    let input = harness.input();
    let last_line = input.lines().last().expect("failed to get last line");

    let mut numbers = input.lines().collect_vec();
    numbers.pop();

    last_line
        .chars()
        .enumerate()
        .map(|(c_index, c)| match c {
            '+' | '*' => {
                let next_c_offset = last_line
                    .chars()
                    .skip(c_index + 1)
                    .position(|c| c == '+' || c == '*');

                let mut col_numbers = vec![];

                match next_c_offset {
                    Some(next_offset) => {
                        for index in c_index..(c_index + next_offset) {
                            let mut col_num = vec![];
                            for row in &numbers {
                                let ch = row.chars().nth(index).unwrap();
                                if ch != ' ' {
                                    col_num.push(ch);
                                }
                            }
                            col_numbers.push(
                                col_num
                                    .into_iter()
                                    .join("")
                                    .parse::<u64>()
                                    .expect("failed to parse"),
                            );
                        }
                    }
                    None => {
                        for index in c_index..last_line.len() {
                            let mut col_num = vec![];
                            for row in &numbers {
                                let ch = row.chars().nth(index).unwrap();
                                if ch != ' ' {
                                    col_num.push(ch);
                                }
                            }
                            col_numbers.push(
                                col_num
                                    .into_iter()
                                    .join("")
                                    .parse::<u64>()
                                    .expect("failed to parse"),
                            );
                        }
                    }
                }

                match c {
                    '+' => col_numbers.iter().sum(),
                    '*' => col_numbers.iter().product(),
                    _ => 0,
                }
            }
            _ => 0,
        })
        .sum::<u64>()
        .to_string()
}
