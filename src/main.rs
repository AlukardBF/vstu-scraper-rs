use std::fs::File;

use prettytable::{cell, row, Table};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plants = page_scraper::scraper().await?;
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
