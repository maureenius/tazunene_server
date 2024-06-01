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
    fn find_by_id(&self, id: u64) -> anyhow::Result<crate::domains::character::Character> {
        todo!()
    }

    fn update(&self, character: crate::domains::character::Character) -> anyhow::Result<()> {
        todo!()
    }
}
