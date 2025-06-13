use crate::database::Conn;
use add::AddArgs;
use clap::Parser;
use del::DelArgs;
use set::SetArgs;

pub mod add;
pub mod del;
pub mod list;
pub mod set;

#[derive(Debug, Parser)]
pub enum Commands {
    Add(AddArgs),
    Set(SetArgs),
    Del(DelArgs),
    List,
}

impl Commands {
    pub async fn run(self, conn: Conn) -> anyhow::Result<()> {
        match self {
            Commands::Add(args) => add::run(args, conn).await,
            Commands::Set(args) => set::run(args, conn).await,
            Commands::Del(args) => del::run(args, conn).await,
            Commands::List => list::run(conn).await,
        }
    }
}
