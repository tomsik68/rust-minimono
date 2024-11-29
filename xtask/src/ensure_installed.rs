use devx_cmd::run;

pub fn main() -> Result<(), anyhow::Error> {
    tracing::info!("Ensuring packages are installed");

    run!("cargo", "install", "cargo-binstall")?;
    run!(
        "cargo",
        "binstall",
        "cargo-deny@=0.16.3",
        "taplo-cli@=0.9.3",
        "cargo-msrv@=0.15.1",
        "cargo-wipe@=0.3.3",
        "typos-cli@=1.16.23",
    )?;

    Ok(())
}
