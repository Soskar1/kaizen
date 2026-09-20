use clap::{Parser, Subcommand};
use thiserror::Error;

mod log;
pub mod status;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct KaizenArgs {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Log(log::LogArgs)
}

#[derive(Error, Debug)]
pub enum KaizenError {
    #[error(transparent)]
    Log(#[from] log::LogError),
}

fn main() -> Result<(), KaizenError> {
    let args = KaizenArgs::parse();
    match args.command {
        Commands::Log(args) => log::log(args)?,
    }

    Ok(())
}
