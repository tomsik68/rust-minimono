use devx_cmd::run;
use tracing::info;

pub fn main() -> Result<(), anyhow::Error> {
    info!("Running taplo fmt");
    run!("taplo", "fmt")?;

    info!("Running cargo fmt");
    run!("cargo", "fmt")?;

    Ok(())
}
