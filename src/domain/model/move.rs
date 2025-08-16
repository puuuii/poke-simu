use crate::domain::model::common::{FlavorText, PokemonTypeName};
use crate::domain::model::stats::StatName;
use serde::{Deserialize, Deserializer, Serialize};

/// Damage class for Pokemon moves
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DamageClass {
    Physical,
    Special,
    Status,
}

impl DamageClass {
    pub fn is_physical(&self) -> bool {
        matches!(self, DamageClass::Physical)
    }

    pub fn is_special(&self) -> bool {
        matches!(self, DamageClass::Special)
    }

    pub fn is_status(&self) -> bool {
        matches!(self, DamageClass::Status)
    }
}

/// Move target enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MoveTarget {
    SelectedPokemon,
    AllOpponents,
    UserOrAlly,
    AllOtherPokemon,
    User,
    RandomOpponent,
    AllPokemon,
    AllAllies,
    UserAndAllies,
    AllFoes,
    SpecificMove,
    EntireBattlefield,
    OpponentsField,
    UserField,
    AllOpponentsField,
    AllPokemonMaybeOthers,
}

impl MoveTarget {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "selected-pokemon" => Some(Self::SelectedPokemon),
            "all-opponents" => Some(Self::AllOpponents),
            "user-or-ally" => Some(Self::UserOrAlly),
            "all-other-pokemon" => Some(Self::AllOtherPokemon),
            "user" => Some(Self::User),
            "random-opponent" => Some(Self::RandomOpponent),
            "all-pokemon" => Some(Self::AllPokemon),
            "all-allies" => Some(Self::AllAllies),
            "user-and-allies" => Some(Self::UserAndAllies),
            "all-foes" => Some(Self::AllFoes),
            "specific-move" => Some(Self::SpecificMove),
            "entire-battlefield" => Some(Self::EntireBattlefield),
            "opponents-field" => Some(Self::OpponentsField),
            "user-field" => Some(Self::UserField),
            "all-opponents-field" => Some(Self::AllOpponentsField),
            "all-pokemon-maybe-others" => Some(Self::AllPokemonMaybeOthers),
            _ => None,
        }
    }
}

/// Move name value object  
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveName {
    pub name: String,
}

impl MoveName {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Move power value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovePower(pub Option<u32>);

impl MovePower {
    pub fn new(power: Option<u32>) -> Self {
        Self(power)
    }

    pub fn value(&self) -> Option<u32> {
        self.0
    }

    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}

/// Move PP (Power Points) value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovePP(pub u32);

impl MovePP {
    pub fn new(pp: u32) -> Self {
        Self(pp)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Move priority value object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MovePriority(pub i32);

impl MovePriority {
    pub fn new(priority: i32) -> Self {
        Self(priority)
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn is_high_priority(&self) -> bool {
        self.0 > 0
    }

    pub fn is_low_priority(&self) -> bool {
        self.0 < 0
    }

    pub fn is_normal_priority(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    pub id: u32,
    pub name: String,
    pub accuracy: Option<u32>,
    #[serde(deserialize_with = "deserialize_damage_class")]
    pub damage_class: DamageClass,
    pub effect_chance: Option<u32>,
    pub effect_entries: Vec<EffectEntry>,
    pub flavor_text_entries: Vec<FlavorText>,
    pub names: Vec<MoveName>,
    pub power: MovePower,
    pub pp: MovePP,
    pub priority: MovePriority,
    pub stat_changes: Vec<StatChange>,
    #[serde(deserialize_with = "deserialize_target")]
    pub target: MoveTarget,
    #[serde(rename = "type")]
    pub type_info: PokemonTypeName,
}

// Helper function to deserialize NamedAPIResource to DamageClass
fn deserialize_damage_class<'de, D>(deserializer: D) -> Result<DamageClass, D::Error>
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
    match resource.name.as_str() {
        "physical" => Ok(DamageClass::Physical),
        "special" => Ok(DamageClass::Special),
        "status" => Ok(DamageClass::Status),
        _ => Err(serde::de::Error::custom(format!(
            "Unknown damage class: {}",
            resource.name
        ))),
    }
}

// Helper function to deserialize NamedAPIResource to MoveTarget
fn deserialize_target<'de, D>(deserializer: D) -> Result<MoveTarget, D::Error>
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
    MoveTarget::from_str(&resource.name)
        .ok_or_else(|| serde::de::Error::custom(format!("Unknown move target: {}", resource.name)))
}

// Helper function to deserialize NamedAPIResource to StatName
fn deserialize_stat<'de, D>(deserializer: D) -> Result<StatName, D::Error>
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
    StatName::from_str(&resource.name)
        .ok_or_else(|| serde::de::Error::custom(format!("Unknown stat name: {}", resource.name)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub short_effect: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatChange {
    pub change: i32,
    #[serde(deserialize_with = "deserialize_stat")]
    pub stat: StatName,
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
        assert_eq!(move_obj.damage_class, DamageClass::Physical);
        assert_eq!(move_obj.target, MoveTarget::SelectedPokemon);
        assert_eq!(move_obj.type_info.name, "normal");
        assert_eq!(move_obj.power.value(), Some(40));
        assert_eq!(move_obj.pp.value(), 35);
        assert_eq!(move_obj.priority.value(), 0);
    }

    #[test]
    fn test_deserialize_status_move() {
        let json_data = r#"
        {
            "id": 45,
            "name": "growl",
            "accuracy": 100,
            "damage_class": { "name": "status", "url": "" },
            "effect_chance": null,
            "effect_entries": [],
            "flavor_text_entries": [],
            "names": [],
            "power": null,
            "pp": 40,
            "priority": 0,
            "stat_changes": [],
            "target": { "name": "all-opponents", "url": "" },
            "type": { "name": "normal", "url": "" }
        }
        "#;

        let move_obj: Move = serde_json::from_str(json_data).unwrap();
        assert_eq!(move_obj.id, 45);
        assert_eq!(move_obj.name, "growl");
        assert_eq!(move_obj.damage_class, DamageClass::Status);
        assert_eq!(move_obj.target, MoveTarget::AllOpponents);
        assert_eq!(move_obj.type_info.name, "normal");
        assert_eq!(move_obj.power.value(), None); // Status moves have no power
        assert_eq!(move_obj.pp.value(), 40);
        assert_eq!(move_obj.priority.value(), 0);
    }
}
