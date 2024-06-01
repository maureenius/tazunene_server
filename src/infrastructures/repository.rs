use crate::domains::character::Character;

pub trait CharacterRepository {
    fn find_by_id(&self, id: u64) -> anyhow::Result<Character>;
    fn update(&self, character: Character) -> anyhow::Result<()>;
}
