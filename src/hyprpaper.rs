use crate::PSW_PATH;
use indoc::formatdoc;
use std::{
    path::{Path, PathBuf},
    process::Stdio,
    sync::LazyLock,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};

static HYPRPAPER_CONFIG: LazyLock<PathBuf> = LazyLock::new(|| PSW_PATH.join("hyprpaper.conf"));

pub async fn set_wallpaper(wallpaper: &Path) -> anyhow::Result<()> {
    let wallpaper = wallpaper.display();

    let config = formatdoc! {r#"
        preload = {wallpaper}
        wallpaper = ,{wallpaper}
        splash = true
        ipc = off
    "#};

    let mut file = File::create(&*HYPRPAPER_CONFIG).await?;
    file.write_all(config.as_bytes()).await?;

    Ok(())
}

pub async fn get_current() -> Option<PathBuf> {
    let mut file = File::open(&*HYPRPAPER_CONFIG).await.ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.ok()?;

    contents
        .lines()
        .find(|line| line.trim().starts_with("wallpaper"))
        .and_then(|line| line.split(',').nth(1))
        .map(|s| PathBuf::from(s.trim()))
}

pub async fn start() -> anyhow::Result<()> {
    Command::new("/bin/killall")
        .arg("hyprpaper")
        .status()
        .await?;

    Command::new("/bin/hyprpaper")
        .args(["-c", &*HYPRPAPER_CONFIG.display().to_string()])
        .stdin(Stdio::null())
        .spawn()?;
    Ok(())
}
