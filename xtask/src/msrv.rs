use devx_cmd::run;

pub fn main() -> Result<(), anyhow::Error> {
    run!("cargo", "msrv")?;
    Ok(())
}
