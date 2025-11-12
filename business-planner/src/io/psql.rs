use sqlx::Postgres;

use crate::io::error::ReadError;

pub async fn read(pool: sqlx::Pool<Postgres>) -> Result<String, ReadError> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(100_i64)
        .fetch_one(&pool).await?;
    Ok(row.0.to_string())
}

#[cfg(test)]
mod tests {
    use pgtemp::{PgTempDBBuilder};
    use sqlx::postgres::PgPoolOptions;

    use super::*;
    
    #[tokio::test]
    async fn test_psql_reader() {
        let builder = PgTempDBBuilder::default().with_bin_path("/usr/lib/postgresql/16/bin");
        let db = builder.start_async().await;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db.connection_uri())
            .await
            .expect("failed to connect to temp db");

        let result = read(pool).await.unwrap();
        assert_eq!(result, "100");
    }
}
