use clap::Parser;
use std::path;

use crate::source;

#[derive(Parser)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    #[arg(
        long,
        value_enum,
        value_name = "some source",
        ignore_case = true,
        default_value = "dummy"
    )]
    pub source: Option<source::ChoiceOfInputSource>,

    #[arg(long, short = 'c', value_name = "path")]
    pub config: Option<path::PathBuf>,

    #[arg(long)]
    pub save: bool,

    #[arg(long)]
    pub disable_timestamps: bool,

    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[arg(short = 'd', long)]
    pub debug: bool,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(short = 'V', long)]
    pub version: bool,
}
