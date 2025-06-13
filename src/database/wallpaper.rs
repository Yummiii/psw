use crate::wallpaper::{Wallpaper, WallpaperMeta};
use lina_rs::macros::repo;
use sqlx::Sqlite;

#[repo(db = Sqlite)]
impl WallpaperRepo {
    pub async fn create(&mut self, wallpaper: &Wallpaper) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO wallpapers (name, data, key) VALUES (?, ?, ?)")
            .bind(&wallpaper.name)
            .bind(&wallpaper.data)
            .bind(&wallpaper.key)
            .execute(&mut *self.conn)
            .await?;
        Ok(())
    }

    pub async fn key_exists(&mut self, key: &str) -> anyhow::Result<bool> {
        Ok(
            sqlx::query_scalar("SELECT EXISTS(SELECT id FROM wallpapers WHERE key = ?)")
                .bind(key)
                .fetch_one(&mut *self.conn)
                .await?,
        )
    }

    pub async fn get_by_key(&mut self, key: &str) -> anyhow::Result<Wallpaper> {
        let wallpaper = sqlx::query_as::<_, Wallpaper>("SELECT * FROM wallpapers WHERE key = ?")
            .bind(key)
            .fetch_one(&mut *self.conn)
            .await?;
        Ok(wallpaper)
    }

    pub async fn delete(&mut self, key: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM wallpapers WHERE key = ?")
            .bind(key)
            .execute(&mut *self.conn)
            .await?;
        Ok(())
    }

    pub async fn list(&mut self) -> anyhow::Result<Vec<WallpaperMeta>> {
        let wallpapers = sqlx::query_as::<_, WallpaperMeta>("SELECT key, name FROM wallpapers")
            .fetch_all(&mut *self.conn)
            .await?;
        Ok(wallpapers)
    }
}
