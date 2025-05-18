use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct Wallpaper {
    pub name: String,
    pub data: Vec<u8>,
    pub key: String,
}

impl Wallpaper {
    pub fn new(data: Vec<u8>, ext: &str, key: String) -> Self {
        let hash = blake3::hash(&data);
        Wallpaper {
            name: format!("{}.{}", hash, ext),
            data,
            key,
        }
    }
}
