use super::stats::StatName;
use crate::domain::model::common::PokemonTypeName;
use crate::domain::model::r#move::MoveName;
use serde::{
    Deserialize, Serialize,
    de::{self, Deserializer},
    ser::{SerializeStruct, Serializer},
};

/// Ability name value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityName {
    pub name: String,
}

impl AbilityName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

fn deserialize_name_from_resource_optional<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct NamedResource {
        name: String,
        #[allow(dead_code)]
        url: String,
    }

    let resource = Option::<NamedResource>::deserialize(deserializer)?;
    Ok(resource.map(|r| r.name))
}

fn deserialize_ability_name<'de, D>(deserializer: D) -> Result<AbilityName, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct NamedResource {
        name: String,
        #[allow(dead_code)]
        url: String,
    }

    let resource = NamedResource::deserialize(deserializer)?;
    Ok(AbilityName::new(resource.name))
}

fn deserialize_move_name<'de, D>(deserializer: D) -> Result<MoveName, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct NamedResource {
        name: String,
        #[allow(dead_code)]
        url: String,
    }

    let resource = NamedResource::deserialize(deserializer)?;
    Ok(MoveName::new(resource.name))
}

fn deserialize_type_name<'de, D>(deserializer: D) -> Result<PokemonTypeName, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct NamedResource {
        name: String,
        #[allow(dead_code)]
        url: String,
    }

    let resource = NamedResource::deserialize(deserializer)?;
    Ok(PokemonTypeName::new(resource.name))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<PokemonForm>,
    pub moves: Vec<PokemonMove>,
    #[serde(deserialize_with = "deserialize_name_from_resource_optional")]
    pub species: Option<String>,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PokemonAbility {
    #[serde(deserialize_with = "deserialize_ability_name")]
    pub ability: AbilityName,
    pub is_hidden: bool,
    pub slot: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PokemonForm {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PokemonMove {
    #[serde(rename = "move", deserialize_with = "deserialize_move_name")]
    pub r#move: MoveName,
}

#[derive(Debug, Clone)]
pub struct PokemonStat {
    pub stat: StatName,
    pub effort: u32,
    pub base_stat: u32,
}

impl<'de> Deserialize<'de> for PokemonStat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct StatHelper {
            name: String,
        }

        #[derive(Deserialize)]
        struct Outer {
            stat: StatHelper,
            effort: u32,
            base_stat: u32,
        }

        let helper = Outer::deserialize(deserializer)?;
        let stat_name = StatName::from_str(&helper.stat.name)
            .ok_or_else(|| de::Error::custom(format!("invalid stat name: {}", helper.stat.name)))?;

        Ok(PokemonStat {
            stat: stat_name,
            effort: helper.effort,
            base_stat: helper.base_stat,
        })
    }
}

impl Serialize for PokemonStat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct StatResource<'a> {
            name: &'a str,
            url: &'a str,
        }

        let mut state = serializer.serialize_struct("PokemonStat", 3)?;
        state.serialize_field(
            "stat",
            &StatResource {
                name: self.stat.as_str(),
                url: "",
            },
        )?;
        state.serialize_field("effort", &self.effort)?;
        state.serialize_field("base_stat", &self.base_stat)?;
        state.end()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PokemonType {
    pub slot: u32,
    #[serde(rename = "type", deserialize_with = "deserialize_type_name")]
    pub type_info: PokemonTypeName,
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
            "abilities": [
                {
                    "ability": {
                        "name": "overgrow",
                        "url": "https://pokeapi.co/api/v2/ability/65/"
                    },
                    "is_hidden": false,
                    "slot": 1
                }
            ],
            "forms": [],
            "moves": [
                {
                    "move": {
                        "name": "tackle",
                        "url": "https://pokeapi.co/api/v2/move/33/"
                    }
                }
            ],
            "species": {
                "name": "bulbasaur",
                "url": "https://pokeapi.co/api/v2/pokemon-species/1/"
            },
            "stats": [
                {
                    "base_stat": 45,
                    "effort": 0,
                    "stat": {
                        "name": "hp",
                        "url": "https://pokeapi.co/api/v2/stat/1/"
                    }
                },
                {
                    "base_stat": 49,
                    "effort": 0,
                    "stat": {
                        "name": "attack",
                        "url": "https://pokeapi.co/api/v2/stat/2/"
                    }
                }
            ],
            "types": [
                {
                    "slot": 1,
                    "type": {
                        "name": "grass",
                        "url": "https://pokeapi.co/api/v2/type/12/"
                    }
                }
            ]
        }
        "#;

        let pokemon: Pokemon = serde_json::from_str(json_data).unwrap();
        assert_eq!(pokemon.id, 1);
        assert_eq!(pokemon.name, "bulbasaur");
        assert_eq!(pokemon.abilities.len(), 1);
        assert_eq!(pokemon.abilities[0].ability.name, "overgrow");
        assert_eq!(pokemon.moves.len(), 1);
        assert_eq!(pokemon.moves[0].r#move.name, "tackle");
        assert_eq!(pokemon.species, Some("bulbasaur".to_string()));
        assert_eq!(pokemon.stats.len(), 2);
        assert_eq!(pokemon.stats[0].stat, StatName::Hp);
        assert_eq!(pokemon.stats[0].base_stat, 45);
        assert_eq!(pokemon.stats[1].stat, StatName::Attack);
        assert_eq!(pokemon.stats[1].base_stat, 49);
        assert_eq!(pokemon.types.len(), 1);
        assert_eq!(pokemon.types[0].type_info.name, "grass");
    }
}
