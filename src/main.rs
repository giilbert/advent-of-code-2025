use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::harness::Harness;

mod day_00_template;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod harness;
mod utils;

fn main() -> anyhow::Result<()> {
    const DEFAULT_LOG_SETTINGS: &str = "advent_of_code_2025=debug";
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .parse(std::env::var("RUST_LOG").unwrap_or(DEFAULT_LOG_SETTINGS.to_string()))?,
        )
        .init();

    let harness = Harness::setup()?;

    let now = std::time::Instant::now();
    let result = match &harness {
        Harness::Run { day, part, .. } => match (day, part) {
            (0, 1) => day_00_template::part_1(&harness),
            (0, 2) => day_00_template::part_2(&harness),
            (1, 1) => day_01::part_1(&harness),
            (1, 2) => day_01::part_2(&harness),
            (2, 1) => day_02::part_1(&harness),
            (2, 2) => day_02::part_2(&harness),
            (3, 1) => day_03::part_1(&harness),
            (3, 2) => day_03::part_2(&harness),
            (4, 1) => day_04::part_1(&harness),
            (4, 2) => day_04::part_2(&harness),
            (5, 1) => day_05::part_1(&harness),
            (5, 2) => day_05::part_2(&harness),
            (6, 1) => day_06::part_1(&harness),
            (6, 2) => day_06::part_2(&harness),
            (7, 1) => day_07::part_1(&harness),
            (7, 2) => day_07::part_2(&harness),
            (8, 1) => day_08::part_1(&harness),
            (8, 2) => day_08::part_2(&harness),
            _ => anyhow::bail!("day {} part {} is not yet implemented", day, part),
        },
        _ => unreachable!(),
    };
    tracing::info!("execution took {:?}", now.elapsed());

    harness.expect(&result)?;

    Ok(())
}
