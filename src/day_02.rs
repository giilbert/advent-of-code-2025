use crate::{harness::Harness, next_tuple};

pub fn part_1(harness: &Harness) -> String {
    harness
        .input()
        .split(",")
        .map(|range| {
            let iter = &mut range.split("-");
            tracing::debug!("{:?}", iter.clone().next());
            let (min, max): (u64, u64) = next_tuple!(iter, u64, u64);

            tracing::debug!(?min, ?max);

            (min..=max)
                .filter(|n| {
                    let s = n.to_string();
                    let (l, r) = s.split_at(s.len() / 2);
                    tracing::debug!(?n, ?l, ?r);
                    l == r
                })
                .inspect(|n| tracing::debug!(?n))
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}

pub fn part_2(harness: &Harness) -> String {
    harness
        .input()
        .split(",")
        .map(|range| {
            let iter = &mut range.split("-");
            tracing::debug!("{:?}", iter.clone().next());
            let (min, max): (u64, u64) = next_tuple!(iter, u64, u64);

            tracing::debug!(?min, ?max);

            (min..=max)
                .filter(|n| {
                    let s = n.to_string();
                    for i in 0..(s.len() / 2) {
                        let pattern = &s[0..=i];
                        if s.trim_start_matches(pattern).is_empty() {
                            tracing::debug!("matched {n:?} {pattern:?}");
                            return true;
                        }
                    }

                    return false;
                })
                .inspect(|n| tracing::debug!(?n))
                .sum::<u64>()
        })
        .sum::<u64>()
        .to_string()
}
