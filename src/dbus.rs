use crate::{
    commands::{
        add::{self, AddArgs},
        del::{self, DelArgs},
        set::{self, SetArgs},
    },
    database::AppPool,
};
use std::{future::pending, path::PathBuf};
use zbus::{conn::Builder, interface};

struct Psw {
    pool: AppPool,
    cfg_dir: PathBuf,
}

#[interface(name = "com.zuraaa.Psw.Controls")]
impl Psw {
    async fn set(&mut self, key: String) {
        set::run(SetArgs { key }, self.pool.clone(), self.cfg_dir.clone())
            .await
            .unwrap();
    }

    async fn add(&mut self, path: PathBuf, key: String) {
        add::run(AddArgs { path, key }, self.pool.clone())
            .await
            .unwrap();
    }

    async fn del(&mut self, key: String) {
        del::run(DelArgs { key }, self.pool.clone()).await.unwrap();
    }
}

pub async fn init_dbus(pool: AppPool, cfg_dir: PathBuf) -> anyhow::Result<()> {
    let psw = Psw { pool, cfg_dir };

    let _conn = Builder::session()?
        .name("com.zuraaa.Psw")?
        .serve_at("/com/zuraaa/Psw", psw)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}
