use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokeType {
    pub id: u32,
    pub name: String,
    pub damage_relations: DamageRelations,
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageRelations {
    pub double_damage_from: Vec<NamedAPIResource>,
    pub double_damage_to: Vec<NamedAPIResource>,
    pub half_damage_from: Vec<NamedAPIResource>,
    pub half_damage_to: Vec<NamedAPIResource>,
    pub no_damage_from: Vec<NamedAPIResource>,
    pub no_damage_to: Vec<NamedAPIResource>,
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
    fn test_deserialize_poke_type() {
        let json_data = r#"
        {
            "id": 10,
            "name": "fire",
            "damage_relations": {
                "no_damage_to": [],
                "half_damage_to": [],
                "double_damage_to": [],
                "no_damage_from": [],
                "half_damage_from": [],
                "double_damage_from": []
            },
            "names": []
        }
        "#;

        let poke_type: PokeType = serde_json::from_str(json_data).unwrap();
        assert_eq!(poke_type.id, 10);
        assert_eq!(poke_type.name, "fire");
    }
}
