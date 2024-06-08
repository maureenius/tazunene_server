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

    async fn find_by_name(&self, name: &CharacterName) -> anyhow::Result<Character> {
        let character_query = r#"SELECT * FROM characters WHERE name = $1"#.to_string();
        let character_record = sqlx::query_as::<_, CharacterRecord>(&character_query)
            .bind(name.as_str())
            .fetch_one(&self.pool)
            .await?;

        let prompt_query = r#"SELECT * FROM prompts WHERE character_id = $1;"#.to_string();
        let prompt_record = sqlx::query_as::<_, PromptRecord>(&prompt_query)
            .bind(character_record.id)
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

    async fn update(&self, old_character: &Character, new_character: &Character) -> anyhow::Result<Character> {
        let tx = self.pool.begin().await?;

        let character_query = r#"UPDATE characters SET name = $1 WHERE name = $2 RETURNING *;"#.to_string();
        let character_record = sqlx::query_as::<_, CharacterRecord>(&character_query)
            .bind(new_character.name.as_str())
            .bind(old_character.name.as_str())
            .fetch_one(&self.pool)
            .await?;

        let prompt_query = r#"UPDATE prompts SET prompt = $1 WHERE character_id = $2 RETURNING *;"#.to_string();
        let prompt_record = sqlx::query_as::<_, PromptRecord>(&prompt_query)
            .bind(new_character.personality.as_str())
            .bind(character_record.id)
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

    #[sqlx::test]
    async fn test_find_by_name() {
        // Setup
        let pool = connect_db().await.unwrap();
        let repo = CharacterRepositoryPg::new(pool);

        let character = Character::new(
            &CharacterName::new("Test Name"),
            &Personality::new("Test Personality"),
        );
        let _ = repo.create(&character).await;

        // Exercise
        let result = repo.find_by_name(&character.name).await;

        // Verify
        assert_eq!(result.unwrap(), character);
    }

    #[sqlx::test]
    async fn test_create() {
        // Setup
        let pool = connect_db().await.unwrap();
        let repo = CharacterRepositoryPg::new(pool);

        let character = Character::new(
            &CharacterName::new("Test Name"),
            &Personality::new("Test Personality"),
        );

        // Exercise
        let result = repo.create(&character).await;

        // Verify
        assert_eq!(result.unwrap(), character);
    }

    #[sqlx::test]
    async fn test_update() {
        // Setup
        let pool = connect_db().await.unwrap();
        let repo = CharacterRepositoryPg::new(pool);

        let old_character = Character::new(
            &CharacterName::new("Test Name"),
            &Personality::new("Test Personality"),
        );
        let _ = repo.create(&old_character).await;

        // Update the character
        let new_character = Character::new(
            &CharacterName::new("Updated Name"),
            &Personality::new("Updated Personality"),
        );

        // Exercise
        let result = repo.update(&old_character, &new_character).await;

        // Verify
        assert!(result.is_ok());

        let updated_character = repo.find_by_name(&new_character.name).await.unwrap();
        assert_eq!(updated_character.name, new_character.name);
        assert_eq!(updated_character.personality, new_character.personality);
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
