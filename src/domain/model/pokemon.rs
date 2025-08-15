use super::r#move::NamedAPIResource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<PokemonForm>,
    pub moves: Vec<PokemonMove>,
    pub species: Option<NamedAPIResource>,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonAbility {
    pub ability: NamedAPIResource,
    pub is_hidden: bool,
    pub slot: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonForm {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub r#move: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonStat {
    pub stat: NamedAPIResource,
    pub effort: u32,
    pub base_stat: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PokemonType {
    pub slot: u32,
    #[serde(rename = "type")]
    pub type_info: NamedAPIResource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_pokemon() {
        let json_data = r#"
        {
            "id": 1,
            "name": "bulbasaur",
            "abilities": [],
            "forms": [],
            "moves": [],
            "species": {
                "name": "bulbasaur",
                "url": "https://pokeapi.co/api/v2/pokemon-species/1/"
            },
            "stats": [],
            "types": []
        }
        "#;

        let pokemon: Pokemon = serde_json::from_str(json_data).unwrap();
        assert_eq!(pokemon.id, 1);
        assert_eq!(pokemon.name, "bulbasaur");
    }
}
