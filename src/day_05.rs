use crate::{harness::Harness, next_tuple};

pub fn part_1(harness: &Harness) -> String {
    struct State {
        ranges: Vec<(u64, u64)>,
    }

    let ranges = &harness
        .input()
        .lines()
        .take_while(|&line| line != "")
        .fold::<State, _>(State { ranges: vec![] }, |mut old_state, line| {
            let mut iter = line.split("-");
            let (start, end) = next_tuple!(&mut iter, u64, u64);

            old_state.ranges.push((start, end));
            old_state
        })
        .ranges;

    harness
        .input()
        .lines()
        .skip_while(|&line| line != "")
        .skip(1)
        .filter(|line| {
            let num: u64 = line.parse().unwrap();

            ranges
                .iter()
                .any(|(start, end)| num >= *start && num <= *end)
        })
        .inspect(|line| println!("valid: {}", line))
        .count()
        .to_string()
}

pub fn part_2(harness: &Harness) -> String {
    struct State {
        ranges: Vec<(u64, u64)>,
    }

    let mut ranges = harness
        .input()
        .lines()
        .take_while(|&line| line != "")
        .fold::<State, _>(State { ranges: vec![] }, |mut old_state, line| {
            let mut iter = line.split("-");
            let (start, end) = next_tuple!(&mut iter, u64, u64);

            old_state.ranges.push((start, end));
            old_state
        })
        .ranges;

    ranges.sort_by_key(|(start, _end)| *start);

    struct State2 {
        count: u64,
        last: Option<(u64, u64)>,
    }

    ranges
        .iter()
        .fold(
            State2 {
                count: 0,
                last: None,
            },
            |mut state, (start, end)| {
                println!(
                    "range: {}-{}, last: {:?}, count: {}",
                    start, end, state.last, state.count
                );

                match state.last {
                    None => {
                        state.last = Some((*start, *end));
                        state.count += end - start + 1;
                        state
                    }
                    Some((last_start, last_end)) => {
                        if *start > last_end {
                            // non-overlapping with the previous range
                            state.count += end - start + 1;
                        } else if *end <= last_end {
                            // fully contained within the previous range
                            // do nothing
                        } else {
                            // partial overlap with the previous range
                            state.count += end - last_end;
                        }

                        state.last =
                            Some((last_start, if *end > last_end { *end } else { last_end }));
                        state
                    }
                }
            },
        )
        .count
        .to_string()
}
