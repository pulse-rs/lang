use clap::{
    builder::{styling, PossibleValuesParser, Styles, TypedValueParser},
    Args, Parser, Subcommand, ValueHint,
};

#[derive(Debug, Parser)]
#[command(author, version, about, name = "pulse",
styles = Styles::styled()
        .header(styling::AnsiColor::Yellow.on_default())
        .usage(styling::AnsiColor::Yellow.on_default())
        .literal(styling::AnsiColor::Green.on_default()))]
pub struct Cli {
    #[arg(global = true, short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Run a file")]
    Run {},

    #[command(about = "Initialize a new project")]
    Init {
        #[arg(name = "NAME")]
        name: Option<String>,
    },
}
