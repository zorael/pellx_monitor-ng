use clap::Parser;
use std::time;

use crate::defaults;

#[derive(Parser)]
pub struct Cli {
    #[arg(short = 'v', long)]
    pub verbose: bool,

    #[arg(short = 'd', long)]
    pub debug: bool,

    #[arg(long)]
    pub dry_run: bool,

    #[arg(short = 'V', long)]
    pub version: bool,
}
