use anyhow::Result;
use cli::Cli;
use handler::setup_panic_handler;
use logger::setup_logger;
use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};

pub mod logger;
pub mod cli;
pub mod handler;

fn main() -> Result<()> {
    setup_panic_handler();
    let args = Cli::parse();
    setup_logger(args.verbose);

    log::debug!("Parsed clap arguments");

    Ok(())
}
