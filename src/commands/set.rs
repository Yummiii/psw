use crate::{
    PSW_PATH,
    database::{Conn, Repos},
    hyprpaper::{get_current, set_wallpaper, start},
};
use clap::Parser;
use tokio::{
    fs::{File, remove_file},
    io::AsyncWriteExt,
};

#[derive(Debug, Parser)]
pub struct SetArgs {
    pub key: String,
}

pub async fn run(args: SetArgs, mut conn: Conn) -> anyhow::Result<()> {
    let mut repo = conn.wallpaper();

    let wallpaper = repo.get_by_key(&args.key).await?;

    if let Some(current) = get_current().await {
        if current.starts_with(&*PSW_PATH) && current.exists() {
            remove_file(current).await?;
        }
    }

    let path = (*PSW_PATH).join(wallpaper.name);

    let mut file = File::create(&path).await?;
    file.write_all(&wallpaper.data).await?;

    set_wallpaper(&path).await?;
    start().await?;

    Ok(())
}
