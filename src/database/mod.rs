use crate::PSW_PATH;
use lina_rs::macros::impl_repos;
use sqlx::{Pool, Sqlite, SqlitePool, migrate, pool::PoolConnection};
use std::path::Path;
use tokio::fs::File;
use wallpaper::WallpaperRepo;

pub mod wallpaper;

pub type Conn = PoolConnection<Sqlite>;

impl_repos!(
    db = Sqlite,
    name = Repos,
    method(name = "wallpaper", repo = WallpaperRepo)
);

pub async fn init_db() -> Pool<Sqlite> {
    let file = (*PSW_PATH).join("wallpapers.db");

    if !Path::new(&file).exists() {
        File::create(&file).await.unwrap();
    }

    let pool = SqlitePool::connect(&format!("sqlite:{}", file.display()))
        .await
        .unwrap();

    migrate!().run(&pool).await.unwrap();

    pool
}
