use clap::Parser;

use crate::source;

#[derive(Parser)]
pub struct Cli {
    #[arg(long = "source", value_enum, ignore_case = true)]
    pub source: Option<source::ChoiceOfInputSource>,

    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[arg(short = 'd', long)]
    pub debug: bool,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(short = 'V', long)]
    pub version: bool,
}
