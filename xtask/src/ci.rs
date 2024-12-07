use devx_cmd::run;
use tracing::info;

pub fn main() -> Result<(), anyhow::Error> {
    info!("Running taplo fmt check");
    run!("taplo", "fmt", "--check")?;

    info!("Running cargo fmt --check");
    run!("cargo", "fmt", "--check")?;

    info!("Running typos");
    run!("typos")?;

    info!("Running cargo deny --check");
    run!("cargo", "deny", "check")?;

    info!("Running cargo clippy");
    run!("cargo", "clippy", "--", "-Dwarnings")?;

    Ok(())
}
