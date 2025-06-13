use crate::database::{Conn, Repos};
use tabled::{
    Table,
    settings::{Reverse, Style},
};

pub async fn run(mut conn: Conn) -> anyhow::Result<()> {
    let mut repo = conn.wallpaper();
    let wallpapers = repo.list().await?;

    let mut table = Table::new(wallpapers);
    table.with(Style::modern());
    table.with(Reverse::columns(0));

    println!("{}", table);
    Ok(())
}
