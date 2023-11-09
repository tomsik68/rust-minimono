use devx_cmd::run;

pub fn main() -> Result<(), anyhow::Error> {
    tracing::info!("Ensuring packages are installed");

    run!(
        "cargo",
        "install",
        "cargo-deny@=0.13.9",
        "taplo-cli@=0.8.0",
        "cargo-msrv@=0.15.1",
        "cargo-wipe@=0.3.3",
    )?;

    Ok(())
}
