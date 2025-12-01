use anyhow::Context;

#[allow(unused)]
use crate::{harness::Harness, next_tuple, utils::IteratorExt};

fn parse_dir_dist(s: &str) -> anyhow::Result<(char, u32)> {
    let direction = s.chars().next().context("missing first character")?;
    let distance: u32 = s[1..].parse()?;
    Ok((direction, distance))
}

pub fn part_1(harness: &Harness) -> anyhow::Result<String> {
    let input = harness.input()?;

    let mut number_of_zeros = 0;
    let mut current_position = 50;
    for line in input.lines() {
        let (direction, distance) = parse_dir_dist(line)?;
        tracing::info!(?direction, ?distance, ?current_position);

        match direction {
            'L' => {
                current_position -= distance as i32;
                if current_position < 0 {
                    while current_position < 0 {
                        current_position += 100;
                    }
                }

                if current_position == 0 {
                    number_of_zeros += 1;
                }
            }
            'R' => {
                current_position += distance as i32;
                if current_position >= 100 {
                    while current_position >= 100 {
                        current_position -= 100;
                    }
                }

                if current_position == 0 {
                    number_of_zeros += 1;
                }
            }
            _ => {
                anyhow::bail!("invalid direction: {}", direction);
            }
        }
    }

    Ok(number_of_zeros.to_string())
}

pub fn part_2(harness: &Harness) -> anyhow::Result<String> {
    let input = harness.input()?;

    let mut number_of_zeros = 0;
    let mut current_position = 50;
    for line in input.lines() {
        let (direction, distance) = parse_dir_dist(line)?;
        tracing::info!(?direction, ?distance, ?current_position);

        match direction {
            'L' => {
                for _ in 0..distance {
                    current_position -= 1;
                    if current_position < 0 {
                        current_position = 99;
                    }

                    if current_position == 0 {
                        number_of_zeros += 1;
                    }
                }
            }
            'R' => {
                for _ in 0..distance {
                    current_position += 1;
                    if current_position >= 100 {
                        current_position = 0;
                    }

                    if current_position == 0 {
                        number_of_zeros += 1;
                    }
                }
            }
            _ => {
                anyhow::bail!("invalid direction: {}", direction);
            }
        }
    }

    Ok(number_of_zeros.to_string())
}
