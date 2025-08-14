use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonSpecies {
    pub id: u32,
    pub name: String,
    pub color: NamedAPIResource,
    pub egg_groups: Vec<NamedAPIResource>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: NamedAPIResource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub language: NamedAPIResource,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
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
