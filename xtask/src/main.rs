use clap::{Parser, Subcommand};

mod ci;
mod ensure_installed;
mod fmt;
mod msrv;
mod wipe;

#[derive(Parser)]
#[command(author, version, about = "Protip: use `cargo xt` instead of `cargo xtask`", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ensures all the right tools are installed
    EnsureInstalled,

    /// Runs CI-level checks for this source code base
    Ci,

    /// Formats all the files that need to be formatted
    Fmt,

    /// Report minimum supported rust version
    Msrv,

    /// Delete unused artifacts from the target directory
    Wipe,
}

fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    tracing::info!("Parsed arguments");

    match cli.command {
        Commands::EnsureInstalled => ensure_installed::main(),
        Commands::Ci => ci::main(),
        Commands::Fmt => fmt::main(),
        Commands::Msrv => msrv::main(),
        Commands::Wipe => wipe::main(),
    }
}
