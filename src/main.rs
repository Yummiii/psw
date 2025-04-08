use std::path::PathBuf;

use arguments::{Cli, Commands};
use clap::Parser;
use commands::{add, daemon, del, set};
use database::init_db;
use dirs::config_dir;
use tokio::fs::create_dir_all;

mod arguments;
mod commands;
mod database;
mod dbus;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let cfg_dir = if cfg!(debug_assertions) {
        PathBuf::from("./")
    } else {
        PathBuf::from(config_dir().unwrap())
    }
    .join("psw");
    create_dir_all(&cfg_dir).await?;

    let pool = init_db(cfg_dir.join("wallpapers.db").display().to_string()).await;

    match args.command {
        Commands::Add(args) => add::run(args, pool).await,
        Commands::Set(args) => set::run(args, pool, cfg_dir).await,
        Commands::Del(args) => del::run(args, pool).await,
        Commands::Daemon => daemon::run(pool, cfg_dir).await,
    }
}
