#![allow(dead_code)]

fn main() {}

/// Simple trait. Can be made deriveable as well to avoid boring boilerplate
trait FromGameState {
    type Output<'a>;

    fn from_gamestate<'a>(gs: &'a GameState) -> Self::Output<'a>;
}

#[derive(Debug, Default)]
struct GameState {
    character: Character,
}

impl GameState {
    fn get<'a, T: FromGameState>(&'a self) -> T::Output<'a> {
        T::from_gamestate(self)
    }
}

#[derive(Debug)]
struct Character {
    name: String, // non-copyable type
    desciption: String,
    level: u8, // copyable type
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: String::from("MyName"),
            desciption: String::from("MyDescription"),
            level: 1,
        }
    }
}

/// We can use existing structs to implement FromGameState to return a reference to the struct
impl FromGameState for Character {
    type Output<'a> = &'a Character;

    fn from_gamestate<'a>(gs: &'a GameState) -> Self::Output<'a> {
        &gs.character
    }
}

/// We can implement a unit struct to return whatever we want
struct Name;

impl FromGameState for Name {
    type Output<'a> = &'a str;

    fn from_gamestate<'a>(gs: &'a GameState) -> Self::Output<'a> {
        &gs.character.name
    }
}

struct Description;

impl FromGameState for Description {
    type Output<'a> = &'a str;

    fn from_gamestate<'a>(gs: &'a GameState) -> Self::Output<'a> {
        &gs.character.desciption
    }
}

/// We dont actually need to return a reference, we can return a copy
struct Level;

impl FromGameState for Level {
    type Output<'a> = u8;

    fn from_gamestate<'a>(gs: &'a GameState) -> Self::Output<'a> {
        gs.character.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copyable() {
        let gs = GameState::default();
        let level = gs.get::<Level>();

        assert_eq!(gs.character.level, level);
    }

    #[test]
    fn multiple_refs() {
        let gs = GameState::default();
        let name = gs.get::<Name>();
        let desc = gs.get::<Description>();

        assert_eq!(gs.character.name, name);
        assert_eq!(gs.character.desciption, desc);
        assert_ne!(name, desc);
    }
}
