use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub effect_entries: Vec<EffectEntry>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub language: NamedAPIResource,
    pub short_effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: NamedAPIResource,
    pub version_group: NamedAPIResource,
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
    fn test_deserialize_ability() {
        let json_data = r#"
        {
            "id": 1,
            "name": "stench",
            "effect_entries": [],
            "flavor_text_entries": [],
            "names": []
        }
        "#;

        let ability: Ability = serde_json::from_str(json_data).unwrap();
        assert_eq!(ability.id, 1);
        assert_eq!(ability.name, "stench");
    }
}
