#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterName(String);
impl CharacterName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
impl From<CharacterName> for String {
    fn from(name: CharacterName) -> Self {
        name.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Personality(String);
impl Personality {
    pub fn new(personality: &str) -> Self {
        Self(personality.to_string())
    }
}
impl From<Personality> for String {
    fn from(personality: Personality) -> Self {
        personality.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character {
    name: CharacterName,
    pub personality: Personality,
}
impl Character {
    pub fn new(name: &CharacterName, personality: &Personality) -> Self {
        Self { name: name.clone(), personality: personality.clone() }
    }
}
