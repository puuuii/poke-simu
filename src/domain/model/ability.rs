use crate::domain::model::common::VersionGroup;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    #[serde(deserialize_with = "deserialize_effect_entries")]
    pub effect_entries: Vec<EffectEntry>,
    #[serde(deserialize_with = "deserialize_flavor_text_entries")]
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    #[serde(deserialize_with = "deserialize_names")]
    pub names: Vec<Name>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub short_effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub version_group: VersionGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
}

// Helper functions for deserialization - filter by language during JSON parsing
fn deserialize_effect_entries<'de, D>(deserializer: D) -> Result<Vec<EffectEntry>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct EffectEntryWithLanguage {
        effect: String,
        short_effect: String,
        language: LanguageResource,
    }

    #[derive(Deserialize)]
    struct LanguageResource {
        name: String,
    }

    let entries: Vec<EffectEntryWithLanguage> = Vec::deserialize(deserializer)?;
    Ok(entries
        .into_iter()
        .filter(|entry| ["ja", "en"].contains(&entry.language.name.as_str()))
        .map(|entry| EffectEntry {
            effect: entry.effect,
            short_effect: entry.short_effect,
        })
        .collect())
}

fn deserialize_flavor_text_entries<'de, D>(
    deserializer: D,
) -> Result<Vec<FlavorTextEntry>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct FlavorTextEntryWithLanguage {
        flavor_text: String,
        language: LanguageResource,
        version_group: VersionGroup,
    }

    #[derive(Deserialize)]
    struct LanguageResource {
        name: String,
    }

    let entries: Vec<FlavorTextEntryWithLanguage> = Vec::deserialize(deserializer)?;
    Ok(entries
        .into_iter()
        .filter(|entry| ["ja", "en"].contains(&entry.language.name.as_str()))
        .map(|entry| FlavorTextEntry {
            flavor_text: entry.flavor_text,
            version_group: entry.version_group,
        })
        .collect())
}

fn deserialize_names<'de, D>(deserializer: D) -> Result<Vec<Name>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct NameWithLanguage {
        name: String,
        language: LanguageResource,
    }

    #[derive(Deserialize)]
    struct LanguageResource {
        name: String,
    }

    let names: Vec<NameWithLanguage> = Vec::deserialize(deserializer)?;
    Ok(names
        .into_iter()
        .filter(|name| ["ja", "en"].contains(&name.language.name.as_str()))
        .map(|name| Name { name: name.name })
        .collect())
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
