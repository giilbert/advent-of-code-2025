use crate::harness::Harness;

pub fn part_1(harness: &Harness) -> anyhow::Result<String> {
    let input = harness.input()?;
    Ok(input.lines().count().to_string())
}

pub fn part_2(harness: &Harness) -> anyhow::Result<String> {
    let input = harness.input()?;
    Ok(input.chars().rev().collect::<String>())
}
