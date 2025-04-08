use crate::{database::AppPool, dbus::init_dbus};
use std::{future::pending, path::PathBuf, process::Stdio};
use tokio::{process::Command, task};

pub async fn run(pool: AppPool, cfg_dir: PathBuf) -> anyhow::Result<()> {
    task::spawn(async { init_dbus(pool, cfg_dir).await.unwrap() });

    task::spawn(async {
        Command::new("/bin/swww")
            .arg("kill")
            .status()
            .await
            .unwrap();

        Command::new("/bin/swww-daemon")
            .stdin(Stdio::null())
            .status()
            .await
            .unwrap();
    });

    pending::<()>().await;

    Ok(())
}
