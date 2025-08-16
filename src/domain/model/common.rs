use serde::{Deserialize, Serialize};

/// Language code value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language {
    pub code: String,
}

impl Language {
    pub fn new(code: String) -> Self {
        Self { code }
    }

    pub fn japanese() -> Self {
        Self {
            code: "ja".to_string(),
        }
    }

    pub fn english() -> Self {
        Self {
            code: "en".to_string(),
        }
    }
}

/// Pokemon type value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonTypeName {
    pub name: String,
}

impl PokemonTypeName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Version value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
}

impl Version {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn scarlet() -> Self {
        Self {
            name: "scarlet".to_string(),
        }
    }

    pub fn violet() -> Self {
        Self {
            name: "violet".to_string(),
        }
    }
}

/// Version group value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionGroup {
    pub name: String,
}

impl VersionGroup {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn scarlet_violet() -> Self {
        Self {
            name: "scarlet-violet".to_string(),
        }
    }
}

// Helper for deserializing from PokeAPI format that contains both name and url
#[derive(Deserialize)]
struct NamedAPIResource {
    name: String,
    #[allow(dead_code)]
    url: Option<String>,
}

impl From<NamedAPIResource> for Language {
    fn from(resource: NamedAPIResource) -> Self {
        Self::new(resource.name)
    }
}

impl From<NamedAPIResource> for PokemonTypeName {
    fn from(resource: NamedAPIResource) -> Self {
        Self::new(resource.name)
    }
}

impl From<NamedAPIResource> for Version {
    fn from(resource: NamedAPIResource) -> Self {
        Self::new(resource.name)
    }
}

impl From<NamedAPIResource> for VersionGroup {
    fn from(resource: NamedAPIResource) -> Self {
        Self::new(resource.name)
    }
}

/// Flavor text value object for moves, abilities, etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: Language,
    pub version_group: VersionGroup,
}

impl FlavorText {
    pub fn new(flavor_text: String, language: Language, version_group: VersionGroup) -> Self {
        Self {
            flavor_text,
            language,
            version_group,
        }
    }
}
