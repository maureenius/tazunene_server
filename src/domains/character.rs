pub struct CharacterName(String);

pub struct Personality(String);

pub struct Character {
    name: CharacterName,
    personality: Personality,
}
impl Character {
    pub fn new(name: CharacterName, personality: Personality) -> Self {
        Self { name, personality }
    }
}
