use crate::{commands::init::init_command, diagnostic::print_diagnostic};
use anyhow::Result;
use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};
use cli::{Cli, Commands};
use commands::run::run_command;
use logger::setup_logger;
use panic_handler::setup_panic_handler;

pub mod ast;
pub mod cli;
pub mod commands;
mod diagnostic;
pub mod error;
pub mod fs;
pub mod lexer;
pub mod logger;
pub mod panic_handler;
pub mod project;
pub mod resolver;
pub mod llvm {
    pub mod ir;
}

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
            log::error!("{:?}", err);
        }
    }

    Ok(())
}
