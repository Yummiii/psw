use arguments::Cli;
use clap::Parser;
use database::init_db;
use dirs::config_dir;
use std::{path::PathBuf, sync::LazyLock};
use tokio::fs::create_dir_all;

mod arguments;
mod commands;
mod database;
mod hyprpaper;
mod wallpaper;

static PSW_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        PathBuf::from("./")
    } else {
        config_dir().unwrap()
    }
    .join("psw")
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    create_dir_all(&*PSW_PATH).await?;

    let pool = init_db().await;
    let conn = pool.acquire().await?;

    args.command.run(conn).await
}
