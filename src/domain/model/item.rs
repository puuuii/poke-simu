use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub cost: Option<u32>,
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
    #[serde(rename = "text")]
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
    fn test_deserialize_item() {
        let json_data = r#"
        {
            "id": 1,
            "name": "master-ball",
            "cost": 0,
            "effect_entries": [],
            "flavor_text_entries": [],
            "names": []
        }
        "#;

        let item: Item = serde_json::from_str(json_data).unwrap();
        assert_eq!(item.id, 1);
        assert_eq!(item.name, "master-ball");
    }
}
