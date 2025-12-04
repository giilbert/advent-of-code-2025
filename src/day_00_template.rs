use crate::harness::Harness;

pub fn part_1(harness: &Harness) -> String {
    let input = harness.input();
    input.lines().count().to_string()
}

pub fn part_2(harness: &Harness) -> String {
    let input = harness.input();
    input.chars().rev().collect::<String>()
}
