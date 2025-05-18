use crate::{
    database::{Conn, Repos},
    wallpaper::Wallpaper,
};
use clap::Parser;
use reqwest::Client;
use sqlx::Acquire;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Parser)]
pub struct AddArgs {
    pub path: PathBuf,
    pub key: String,
}

pub async fn run(args: AddArgs, mut conn: Conn) -> anyhow::Result<()> {
    let mut tx = conn.begin().await?;
    let mut repo = tx.wallpaper();

    if repo.key_exists(&args.key).await? {
        println!("Overwriting existing key: {}", args.key);
        repo.delete(&args.key).await?;
    }

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

    let ext = args
        .path
        .extension()
        .and_then(|s| s.to_str())
        .expect("Failed to get file extension");

    repo.create(&Wallpaper::new(img, ext, args.key)).await?;
    tx.commit().await?;

    Ok(())
}
