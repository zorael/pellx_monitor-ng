use clap::Parser;
use std::path;

use crate::source;

#[derive(Parser)]
#[allow(clippy::struct_excessive_bools)]
pub struct Cli {
    /// Input source to poll
    #[arg(
        long,
        value_enum,
        value_name = "some source",
        ignore_case = true,
        default_value = "dummy"
    )]
    pub source: Option<source::ChoiceOfInputSource>,

    /// Specify an alternative configuration file
    #[arg(long, short = 'c', value_name = "path")]
    pub config: Option<path::PathBuf>,

    /// Write configuration to disk
    #[arg(long)]
    pub save: bool,

    /// Disable timestamps in terminal output
    #[arg(long)]
    pub disable_timestamps: bool,

    /// Print some additional information
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Print much more additional information
    #[arg(short = 'd', long)]
    pub debug: bool,

    /// Perform a dry run, echoing what would be done
    #[arg(long)]
    pub dry_run: bool,

    /// Display version information and exit
    #[arg(short = 'V', long)]
    pub version: bool,
}
