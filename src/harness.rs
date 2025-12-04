use anyhow::Context;
use clap::Parser;

const PROJECT_ROOT: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Debug, clap::Parser)]
pub enum Harness {
    /// Create a new day's scaffolding
    Create { day: u32 },
    /// Run the code on the real input
    Run { day: u32, part: u8, file: String },
}

impl Harness {
    /// Create a new [`Harness`] by parsing command line arguments and possibly running setup code.
    pub fn setup() -> anyhow::Result<Self> {
        let harness = Harness::parse();

        match &harness {
            Harness::Create { day } => {
                tracing::info!("creating scaffolding for day {}", day);

                let dir = format!("{}/tests/day-{:02}", PROJECT_ROOT, day);
                tracing::info!("created day directory: {}", dir);
                std::fs::create_dir_all(dir).context("Failed to create day directory")?;

                const NUM_TEST_FILES: u32 = 2;
                for index in 1..=NUM_TEST_FILES {
                    let path = format!("{}/tests/day-{:02}/input-{}.txt", PROJECT_ROOT, day, index);
                    tracing::info!("created test input file: {}", path);
                    std::fs::write(path, "expected: <skip>\ncomments:\n<insert input>")
                        .context("Failed to create test input file")?;
                }

                std::process::exit(0);
            }
            Harness::Run { .. } => (),
        }

        Ok(harness)
    }

    /// Read the full input from the file specified in the [`Harness`].
    pub fn full_input(&self) -> anyhow::Result<String> {
        match self {
            Harness::Run { day, file, .. } => {
                let input = std::fs::read_to_string(format!(
                    "{}/tests/day-{:02}/{}.txt",
                    PROJECT_ROOT, day, file
                ))
                .with_context(|| format!("Failed to read input file: {}", file))?;
                Ok(input)
            }
            _ => anyhow::bail!("input can only be read in Run mode"),
        }
    }

    /// Get the input stripped of expected output and comments.
    pub fn input(&self) -> String {
        let full_input = self.full_input().expect("failed to read full input");
        let input = full_input.lines().skip(2).collect::<Vec<_>>().join("\n");
        input
    }

    /// Check the provided output against the expected output in the input file.
    pub fn expect(&self, got: &str) -> anyhow::Result<()> {
        let full_input = self.full_input()?;
        let expected_line = full_input.lines().next().unwrap_or("");
        let expected = expected_line
            .strip_prefix("expected: ")
            .unwrap_or("<skip>")
            .trim();

        if expected == "<real>" {
            tracing::info!("<real> output: {}", got);
            return Ok(());
        }

        if expected == "<skip>" {
            tracing::warn!("no expected output provided; skipping check");
            return Ok(());
        }

        if expected == got.trim() {
            tracing::info!("output matches expected");
            Ok(())
        } else {
            tracing::error!(
                "output does not match:\n     expected: {}\n          got: {}",
                expected,
                got
            );
            Ok(())
        }
    }
}
