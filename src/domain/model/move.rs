use serde::{Deserialize, Serialize};

// `pokemon.rs` など他のモデルでも利用される共通構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NamedAPIResource {
    pub name: String,
    #[serde(default)]
    #[allow(dead_code)]
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    pub id: u32,
    pub name: String,
    pub accuracy: Option<u32>,
    pub damage_class: NamedAPIResource,
    pub effect_chance: Option<u32>,
    pub effect_entries: Vec<EffectEntry>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub generation: NamedAPIResource,
    pub names: Vec<MoveName>,
    pub power: Option<u32>,
    pub pp: Option<u32>,
    pub priority: i32,
    pub stat_changes: Vec<StatChange>,
    pub target: NamedAPIResource,
    #[serde(rename = "type")]
    pub type_info: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub language: NamedAPIResource,
    pub short_effect: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: NamedAPIResource,
    pub version_group: NamedAPIResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoveName {
    pub language: NamedAPIResource,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatChange {
    pub change: i32,
    pub stat: NamedAPIResource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_move() {
        let json_data = r#"
        {
            "id": 1,
            "name": "pound",
            "accuracy": 100,
            "damage_class": { "name": "physical", "url": "" },
            "effect_chance": null,
            "effect_entries": [],
            "flavor_text_entries": [],
            "generation": { "name": "generation-i", "url": "" },
            "names": [],
            "power": 40,
            "pp": 35,
            "priority": 0,
            "stat_changes": [],
            "target": { "name": "selected-pokemon", "url": "" },
            "type": { "name": "normal", "url": "" }
        }
        "#;

        let move_obj: Move = serde_json::from_str(json_data).unwrap();
        assert_eq!(move_obj.id, 1);
        assert_eq!(move_obj.name, "pound");
    }
}
