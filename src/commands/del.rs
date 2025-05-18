use crate::database::{Conn, Repos};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct DelArgs {
    pub key: String,
}

pub async fn run(args: DelArgs, mut conn: Conn) -> anyhow::Result<()> {
    let mut repo = conn.wallpaper();
    repo.delete(&args.key).await?;
    Ok(())
}
