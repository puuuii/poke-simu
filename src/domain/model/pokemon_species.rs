use crate::domain::model::common::Version;
use serde::{Deserialize, Deserializer, Serialize};

/// Pokemon color enum for species
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PokemonColor {
    Black,
    Blue,
    Brown,
    Gray,
    Green,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
}

impl PokemonColor {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "black" => Some(Self::Black),
            "blue" => Some(Self::Blue),
            "brown" => Some(Self::Brown),
            "gray" => Some(Self::Gray),
            "green" => Some(Self::Green),
            "pink" => Some(Self::Pink),
            "purple" => Some(Self::Purple),
            "red" => Some(Self::Red),
            "white" => Some(Self::White),
            "yellow" => Some(Self::Yellow),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpecies {
    pub id: u32,
    pub name: String,
    #[serde(deserialize_with = "deserialize_pokemon_color")]
    pub color: PokemonColor,
    #[serde(deserialize_with = "deserialize_names_from_resources")]
    pub egg_groups: Vec<String>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
}

// Helper functions for deserialization
fn deserialize_pokemon_color<'de, D>(deserializer: D) -> Result<PokemonColor, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct NamedAPIResource {
        name: String,
        #[allow(dead_code)]
        url: Option<String>,
    }

    let resource = NamedAPIResource::deserialize(deserializer)?;
    PokemonColor::from_str(&resource.name).ok_or_else(|| {
        serde::de::Error::custom(format!("Unknown pokemon color: {}", resource.name))
    })
}

fn deserialize_names_from_resources<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct NamedAPIResource {
        name: String,
        #[allow(dead_code)]
        url: Option<String>,
    }

    let resources: Vec<NamedAPIResource> = Vec::deserialize(deserializer)?;
    Ok(resources.into_iter().map(|r| r.name).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_pokemon_species() {
        let json_data = r#"
        {
            "id": 1,
            "name": "bulbasaur",
            "color": {
                "name": "green",
                "url": "https://pokeapi.co/api/v2/pokemon-color/5/"
            },
            "egg_groups": [],
            "flavor_text_entries": [],
            "names": []
        }
        "#;

        let species: PokemonSpecies = serde_json::from_str(json_data).unwrap();
        assert_eq!(species.id, 1);
        assert_eq!(species.name, "bulbasaur");
    }
}
