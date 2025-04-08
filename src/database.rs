use sqlx::{Pool, Sqlite, SqlitePool, migrate, prelude::FromRow};
use std::{path::Path, sync::Arc};
use tokio::fs::File;

#[derive(FromRow)]
pub struct Wallpaper {
    pub id: i32,
    pub name: String,
    pub data: Vec<u8>,
}

pub type AppPool = Arc<Pool<Sqlite>>;

pub async fn init_db(file: String) -> AppPool {
    if !Path::new(&file).exists() {
        File::create(&file).await.unwrap();
    }

    let pool = SqlitePool::connect(&format!("sqlite:{}", file))
        .await
        .unwrap();

    migrate!().run(&pool).await.unwrap();

    Arc::new(pool)
}
