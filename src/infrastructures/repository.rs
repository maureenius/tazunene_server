use anyhow::Ok;
use chrono::{DateTime, Local};
use sqlx::{PgPool, types::chrono};

use crate::domains::{character::{Character, CharacterName, Personality}, infra_trait::CharacterRepository};

pub struct CharacterRepositoryPg {
    pool: PgPool,
}
impl CharacterRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl CharacterRepository for CharacterRepositoryPg {
    async fn find_by_id(&self, id: u64) -> anyhow::Result<Character> {
        let character_query = r#"SELECT * FROM characters WHERE id = $1"#.to_string();
        let character_record = sqlx::query_as::<_, CharacterRecord>(&character_query)
            .bind(id as i32)
            .fetch_one(&self.pool)
            .await?;

        let prompt_query = r#"SELECT * FROM prompts WHERE character_id = $1;"#.to_string();
        let prompt_record = sqlx::query_as::<_, PromptRecord>(&prompt_query)
            .bind(id as i32)
            .fetch_one(&self.pool)
            .await?;

        Ok(
            Character::new(
            &CharacterName::new(&character_record.name),
            &Personality::new(&prompt_record.prompt),
            )
        )
    }

    async fn create(&self, character: &Character) -> anyhow::Result<Character> {
        let tx = self.pool.begin().await?;
        let character_query = r#"INSERT INTO characters (name) VALUES ($1) RETURNING *;"#.to_string();
        let character_record = sqlx::query_as::<_, CharacterRecord>(&character_query)
            .bind(character.name.as_str())
            .fetch_one(&self.pool)
            .await?;

        let prompt_query = r#"INSERT INTO prompts (character_id, prompt) VALUES ($1, $2) RETURNING *;"#.to_string();
        let prompt_record = sqlx::query_as::<_, PromptRecord>(&prompt_query)
            .bind(character_record.id)
            .bind(character.personality.as_str())
            .fetch_one(&self.pool)
            .await?;

        tx.commit().await?;

        Ok(
            Character::new(
                &CharacterName::new(&character_record.name),
                &Personality::new(&prompt_record.prompt),
            )
        )
    }

    async fn update(&self, character: &Character) -> anyhow::Result<Character> {
        todo!()
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct CharacterRecord {
    id: i32,
    name: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct PromptRecord {
    id: i32,
    character_id: i32,
    prompt: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::postgres::PgPoolOptions;

    use crate::domains::character::Character;
    use crate::domains::infra_trait::CharacterRepository;

    use super::*;

    #[sqlx::test]
    async fn test_find_by_id() {
        // Setup
        let pool = connect_db().await.unwrap();
        let repo = CharacterRepositoryPg::new(pool);

        let character = Character::new(
            &CharacterName::new("Test Name"),
            &Personality::new("Test Personality"),
        );
        let _ = repo.create(&character).await;

        // Exercise
        let result = repo.find_by_id(1).await;

        // Verify
        assert_eq!(result.unwrap(), character);
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
