use devx_cmd::run;

pub fn main() -> Result<(), anyhow::Error> {
    run!("cargo", "wipe", "rust", "-w")?;
    Ok(())
}
