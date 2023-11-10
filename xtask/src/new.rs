use anyhow::Context;
use devx_cmd::{cmd, read};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use tracing::info;

#[derive(clap::Args)]
pub struct Command {
    pub crate_name: String,

    #[arg(long)]
    bin: bool,
}

pub fn main(cmd: Command) -> Result<(), anyhow::Error> {
    info!("Setting up new Rust crate...");

    let manifest_path = read!(
        "cargo",
        "locate-project",
        "--workspace",
        "--message-format=plain"
    )?
    .trim()
    .to_owned();

    let mut cargo_new = cmd!(
        "cargo",
        "new",
        match cmd.bin {
            true => "--bin",
            false => "--lib",
        },
        &cmd.crate_name,
    );

    let crates_dir = {
        let mut path = PathBuf::from(&manifest_path);

        // pop the Cargo.toml
        path.pop();
        path.push("crates");
        path
    };

    let mut val: toml::Value = {
        let mut file = File::open(&manifest_path)
            .with_context(|| format!("while opening {}", &manifest_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .with_context(|| format!("while reading {}", &manifest_path))?;

        toml::from_str(&contents).with_context(|| format!("While parsing {}", &manifest_path))?
    };

    let root = val.as_table_mut().ok_or_else(|| {
        anyhow::anyhow!(
            "Unable to convert the toml file {} to a table",
            &manifest_path
        )
    })?;

    let workspace = root
        .get_mut("workspace")
        .ok_or_else(|| anyhow::anyhow!("Unable to find workspace key in {}", &manifest_path))?
        .as_table_mut()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Unable to convert workspace key in {} to a table",
                &manifest_path
            )
        })?;

    let members = workspace
        .get_mut("members")
        .ok_or_else(|| {
            anyhow::anyhow!("Unable to find workspace.members key in {}", &manifest_path)
        })?
        .as_array_mut()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Unable to convert workspace.members key in {} to an array",
                &manifest_path
            )
        })?;

    members.push(format!("crates/{}", &cmd.crate_name).into());

    {
        let mut file = File::create(&manifest_path)
            .with_context(|| format!("While opening {}", &manifest_path))?;
        let contents = toml::to_string(&val)?;
        file.write_all(contents.as_bytes())
            .with_context(|| format!("While writing {}", &manifest_path))?;
    }

    cargo_new.current_dir(crates_dir);
    cargo_new.run()?;

    Ok(())
}
