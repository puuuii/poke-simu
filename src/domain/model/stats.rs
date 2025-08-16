#[derive(Debug, PartialEq, Eq, Hash, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StatName {
    Hp,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}

impl StatName {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "hp" => Some(Self::Hp),
            "attack" => Some(Self::Attack),
            "defense" => Some(Self::Defense),
            "special-attack" => Some(Self::SpecialAttack),
            "special-defense" => Some(Self::SpecialDefense),
            "speed" => Some(Self::Speed),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hp => "hp",
            Self::Attack => "attack",
            Self::Defense => "defense",
            Self::SpecialAttack => "special-attack",
            Self::SpecialDefense => "special-defense",
            Self::Speed => "speed",
        }
    }
}
