use crate::database::AppPool;
use clap::Parser;
use reqwest::Client;
use sqlx::query;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Parser)]
pub struct AddArgs {
    pub path: PathBuf,
    pub key: String,
}

pub async fn run(args: AddArgs, pool: AppPool) -> anyhow::Result<()> {
    let img = if args.path.exists() {
        let mut buf = vec![];
        File::open(&args.path).await?.read_to_end(&mut buf).await?;
        buf
    } else {
        Client::new()
            .get(args.path.display().to_string())
            .header("User-Agent", "mikumiku")
            .send()
            .await?
            .bytes()
            .await?
            .to_vec()
    };

    query("INSERT INTO wallpapers (key, name, data) VALUES (?, ? ,?)")
        .bind(args.key)
        .bind(args.path.file_name().unwrap().to_str().unwrap())
        .bind(img)
        .execute(&*pool)
        .await?;

    Ok(())
}
