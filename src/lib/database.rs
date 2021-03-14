use crate::{Houseplant, OptArg};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePoolOptions;

#[async_trait]
pub trait Database {
    async fn insert(&self, plant: &Houseplant) -> Result<()>;
}

pub struct Sqlite {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

impl Sqlite {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new().connect(database_url).await?;
        let mut conn = pool.acquire().await?;
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS plants
            (
                name TEXT NOT NULL,
                image TEXT,
                temperature TEXT,
                humidity TEXT,
                illumination TEXT,
                watering TEXT,
                soil TEXT,
                fertilizer TEXT,
                transplant TEXT,
                propagation TEXT,
                features TEXT
            );
            "#
        )
        .execute(&mut conn)
        .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl Database for Sqlite {
    async fn insert(&self, plant: &Houseplant) -> Result<()> {
        let attrs = &plant.attributes;
        let temperature = attrs.temperature.get_value();
        let humidity = attrs.humidity.get_value();
        let illumination = attrs.illumination.get_value();
        let watering = attrs.watering.get_value();
        let soil = attrs.soil.get_value();
        let fertilizer = attrs.fertilizer.get_value();
        let transplant = attrs.transplant.get_value();
        let propagation = attrs.propagation.get_value();
        let features = attrs.features.get_value();

        let mut conn = self.pool.acquire().await?;
        let _res = sqlx::query!(
            r#"
            INSERT INTO plants (
                name, image, temperature, humidity, illumination, watering,
                soil, fertilizer, transplant, propagation, features
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            plant.name,
            plant.image,
            temperature,
            humidity,
            illumination,
            watering,
            soil,
            fertilizer,
            transplant,
            propagation,
            features
        )
        .execute(&mut conn)
        .await?;
        Ok(())
    }
}
