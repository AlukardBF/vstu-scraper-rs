use prettytable::{cell, row, Table};
use std::fs::File;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plants = page_scraper::Scraper::new(10).scraper().await?;
    let mut file = File::create("result.txt")?;
    plants.iter().for_each(|plant| {
        let mut table = Table::new();
        table.add_row(row!["Название", plant.name]);
        plant.attributes.iter().for_each(|attr| {
            table.add_row(row![attr.parameter, attr.value]);
        });
        table.print(&mut file).unwrap();
    });
    Ok(())
}
