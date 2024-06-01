pub struct CharacterName(String);

pub struct Personality(String);
impl From<Personality> for String {
    fn from(personality: Personality) -> Self {
        personality.0
    }
}

pub struct Character {
    name: CharacterName,
    pub personality: Personality,
}
impl Character {
    pub fn new(name: CharacterName, personality: Personality) -> Self {
        Self { name, personality }
    }
}
