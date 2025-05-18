use clap::Parser;

use crate::commands::{add::AddArgs, del::DelArgs, set::SetArgs};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    Add(AddArgs),
    Set(SetArgs),
    Del(DelArgs),
}
