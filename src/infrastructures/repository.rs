use sqlx::PgPool;

use crate::domains::infra_trait::CharacterRepository;

pub struct CharacterRepositoryPg {
    pool: PgPool,
}
impl CharacterRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl CharacterRepository for CharacterRepositoryPg {
    async fn find_by_id(&self, id: u64) -> anyhow::Result<crate::domains::character::Character> {
        todo!()
    }

    async fn update(&self, character: crate::domains::character::Character) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Pool, Postgres};
    use crate::domains::character::Character;
    use crate::domains::infra_trait::CharacterRepository;

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = connect_db().await.unwrap();
        let repo = CharacterRepositoryPg::new(pool);
        let result = repo.find_by_id(1).await;
        // Assert based on your test database data
        assert!(result.is_ok());
    }


    async fn connect_db() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL_TEST").expect("undefined [DATABASE_URL_TEST]");
    
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
    }
}
