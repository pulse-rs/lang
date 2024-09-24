use anyhow::Result;
use cli::{Cli, Commands};
use commands::run::run_command;
use handler::setup_panic_handler;
use logger::setup_logger;
use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};

pub mod commands;
pub mod logger;
pub mod cli;
pub mod handler;
pub mod fs;

fn main() -> Result<()> {
    setup_panic_handler();
    let args = Cli::parse();
    setup_logger(args.verbose);

    log::debug!("Parsed clap arguments");

    let result = match args.command {
        Commands::Run {} => run_command() 
    };

    match result {
        Ok(_) => {
            log::debug!("Finished program")
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    Ok(())
}
