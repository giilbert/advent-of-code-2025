use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::harness::Harness;

mod day_00_template;
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
            (0, 1) => day_00_template::part_1(&harness)?,
            (0, 2) => day_00_template::part_2(&harness)?,
            _ => anyhow::bail!("day {} part {} is not yet implemented", day, part),
        },
        _ => unreachable!(),
    };
    tracing::info!("execution took {:?}", now.elapsed());

    harness.expect(&result)?;

    Ok(())
}
