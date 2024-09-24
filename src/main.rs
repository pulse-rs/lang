use anyhow::Result;
use cli::{Cli, Commands};
use commands::run::run_command;
use panic_handler::setup_panic_handler;
use logger::setup_logger;
use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};
use crate::commands::init::init_command;

pub mod commands;
pub mod logger;
pub mod cli;
pub mod panic_handler;
pub mod fs;
pub mod project;
pub mod error;
mod lexer;
mod diagnostic;

fn main() -> Result<()> {
    setup_panic_handler();
    let args = Cli::parse();
    setup_logger(args.verbose);

    log::debug!("Parsed clap arguments");

    let result = match args.command {
        Commands::Run {} => run_command(),
        Commands::Init { name } => init_command(name.clone()),
    };

    match result {
        Ok(_) => {
            log::debug!("Finished program")
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    Ok(())
}
