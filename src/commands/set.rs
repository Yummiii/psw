use crate::database::{AppPool, Wallpaper};
use clap::Parser;
use sqlx::{query, query_as, query_scalar};
use std::{path::PathBuf, process::Command};
use tokio::{
    fs::{File, remove_file},
    io::AsyncWriteExt,
};

#[derive(Debug, Parser)]
pub struct SetArgs {
    pub key: String,
}

pub async fn run(args: SetArgs, pool: AppPool, cfg_dir: PathBuf) -> anyhow::Result<()> {
    let current =
        query_scalar::<_, String>("SELECT name FROM wallpapers WHERE current = true LIMIT 1")
            .fetch_optional(&*pool)
            .await?;

    if let Some(current) = current {
        remove_file(cfg_dir.join(current)).await?;
    }

    let wallpaper = query_as::<_, Wallpaper>(
        "SELECT id, name, data FROM wallpapers WHERE key = ? ORDER BY RANDOM() LIMIT 1",
    )
    .bind(args.key)
    .fetch_one(&*pool)
    .await?;

    let path = cfg_dir.join(wallpaper.name);

    let mut file = File::create(&path).await?;
    file.write_all(&wallpaper.data).await?;

    query("UPDATE wallpapers SET current = false WHERE current = true")
        .execute(&*pool)
        .await?;

    query("UPDATE wallpapers SET current = true WHERE id = ?")
        .bind(wallpaper.id)
        .execute(&*pool)
        .await?;

    Command::new("/bin/swww")
        .args(["img", &path.display().to_string(), "-t", "none"])
        .status()
        .unwrap();

    Ok(())
}
