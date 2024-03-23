use sqlx::Error;

#[derive(Clone)]
pub struct DbConfig {
    url: String,
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl DbConfig {
    pub async fn new(url: String) -> Self {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("Failed to create DB pool.");

        DbConfig {
            url,
            pool
        }

    }

    pub async fn migrate(&self) -> Result<(), Error> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }

    pub async fn write_cc_info(&self, card_details: &super::structures::CardDetailsDb) -> Result<(), Error> {
        let query: &str = "INSERT INTO prepaid_cards (card_number, expiration_month, expiration_year, security_code) VALUES ($1, $2, $3, $4)";

        sqlx::query(query)
            .bind(&card_details.card_number)
            .bind(&card_details.expiration_month)
            .bind(&card_details.expiration_year)
            .bind(&card_details.security_code)
            .execute(&self.pool)
            .await?;

        // TODO: Figure out error mapping

        Ok(())
    }
}