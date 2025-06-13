use sqlx::prelude::FromRow;
use tabled::Tabled;

#[derive(FromRow)]
pub struct Wallpaper {
    pub name: String,
    pub data: Option<Vec<u8>>,
    pub key: String,
}

#[derive(FromRow, Tabled)]
pub struct WallpaperMeta {
    pub name: String,
    pub key: String,
}

impl Wallpaper {
    pub fn new(data: Vec<u8>, ext: &str, key: String) -> Self {
        let hash = blake3::hash(&data);
        Wallpaper {
            name: format!("{}.{}", hash, ext),
            data: Some(data),
            key,
        }
    }
}
