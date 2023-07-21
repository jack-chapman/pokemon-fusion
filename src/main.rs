use std::{collections::HashMap, ops::Add};

struct PokemonDb {
    names: HashMap<usize, String>,
    prefixes: HashMap<usize, String>,
    suffixes: HashMap<usize, String>,
}

impl PokemonDb {
    fn try_load() -> Result<Self, std::io::Error> {
        let names: HashMap<usize, String> = serde_json::from_str(include_str!("names.json"))?;

        let prefixes: HashMap<usize, String> = serde_json::from_str(include_str!("prefixes.json"))?;

        let suffixes: HashMap<usize, String> = serde_json::from_str(include_str!("suffixes.json"))?;

        Ok(Self {
            names,
            prefixes,
            suffixes,
        })
    }

    fn name(&self, id: usize) -> Option<String> {
        self.names.get(&id).cloned()
    }

    fn prefix(&self, id: usize) -> Option<String> {
        self.prefixes.get(&id).cloned()
    }

    fn suffix(&self, id: usize) -> Option<String> {
        self.suffixes.get(&id).cloned()
    }

    fn pokemon(&self, id: usize) -> Option<Pokemon> {
        let name = self.name(id)?;
        let prefix = self.prefix(id)?;
        let suffix = self.suffix(id)?;

        Some(Pokemon::new(name, prefix, suffix))
    }
}

#[derive(Clone)]
struct Pokemon {
    name: String,
    prefix: String,
    suffix: String,
}

impl Pokemon {
    fn new(name: String, prefix: String, suffix: String) -> Self {
        Self {
            name,
            prefix,
            suffix,
        }
    }

    fn fuse_with(&self, other: &Self) -> Self {
        let prefix = &self.prefix;
        let suffix = &other.suffix;
        let name = format!("{}{}", prefix, suffix);

        Self::new(name, prefix.to_owned(), suffix.to_owned())
    }
}

impl Add for Pokemon {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.fuse_with(&rhs)
    }
}

fn main() {
    let pokemon_db = PokemonDb::try_load().expect("Unable to load pokemon database.");

    let oddish = pokemon_db.pokemon(43).unwrap();

    let snorlax = pokemon_db.pokemon(143).unwrap();

    let snorish = snorlax + oddish;
    assert_eq!(snorish.name, "Snorish");

    // uncomment this and you'll see the horrible truth about pokemon fusion
    // let oddlax = oddish + snorlax;
    // assert_eq!(oddlax.name, "Oddlax");
}
