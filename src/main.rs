use page_scraper::database;
use std::env;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let database_url = &env::var("DATABASE_URL").unwrap_or("sqlite:plants.db".to_string());
    let db = database::Sqlite::new(database_url).await?;
    let _plants = page_scraper::Scraper::new(10, Some(db)).scraper().await?;
    Ok(())
}
