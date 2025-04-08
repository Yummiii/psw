use clap::Parser;
use sqlx::query;

use crate::database::AppPool;

#[derive(Debug, Parser)]
pub struct DelArgs {
    pub key: String,
}

pub async fn run(args: DelArgs, pool: AppPool) -> anyhow::Result<()> {
    query("DELETE FROM wallpapers WHERE \"key\"= ?")
        .bind(args.key)
        .execute(&*pool)
        .await?;

    Ok(())
}
