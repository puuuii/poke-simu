use crate::domain::model::ability::Ability;
use crate::domain::model::pokemon::Pokemon;
use crate::domain::model::stats::StatName;
use crate::domain::model::status::StatusCondition;

#[derive(Clone, Debug)]
pub struct ActivePokemon {
    // Pokemon由来の不変に近い情報
    pub name: String,
    pub active_ability: Ability,

    // バトル中の変動ステータス
    pub current_hp: u32,
    pub max_hp: u32,
    pub status: Option<StatusCondition>,

    // 能力値
    pub attack: u32,
    pub defense: u32,
    pub special_attack: u32,
    pub special_defense: u32,
    pub speed: u32,

    // 能力ランク (-6 ~ +6)
    pub attack_rank: i8,
    pub defense_rank: i8,
    pub special_attack_rank: i8,
    pub special_defense_rank: i8,
    pub speed_rank: i8,
    pub accuracy_rank: i8,
    pub evasion_rank: i8,
}

impl ActivePokemon {
    pub fn new(pokemon: &Pokemon, ability: &Ability) -> Self {
        let get_stat = |stat_name: StatName| -> u32 {
            pokemon
                .stats
                .iter()
                .find(|s| s.stat == stat_name)
                .map(|s| s.base_stat)
                .unwrap_or_else(|| panic!("'{:?}' stat not found for {}", stat_name, pokemon.name))
        };

        let max_hp = get_stat(StatName::Hp);

        Self {
            name: pokemon.name.clone(),
            active_ability: ability.clone(),
            max_hp,
            current_hp: max_hp,
            status: None,
            attack: get_stat(StatName::Attack),
            defense: get_stat(StatName::Defense),
            special_attack: get_stat(StatName::SpecialAttack),
            special_defense: get_stat(StatName::SpecialDefense),
            speed: get_stat(StatName::Speed),
            attack_rank: 0,
            defense_rank: 0,
            special_attack_rank: 0,
            special_defense_rank: 0,
            speed_rank: 0,
            accuracy_rank: 0,
            evasion_rank: 0,
        }
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.current_hp = self.current_hp.saturating_sub(damage);
    }

    pub fn is_fainted(&self) -> bool {
        self.current_hp == 0
    }

    pub fn set_status(&mut self, status: StatusCondition) {
        self.status = Some(status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::ability::Ability;
    use crate::domain::model::pokemon::{Pokemon, PokemonStat};

    fn create_test_pokemon() -> Pokemon {
        Pokemon {
            id: 1,
            name: "Testmon".to_string(),
            abilities: vec![],
            forms: vec![],
            moves: vec![],
            species: None,
            stats: vec![
                PokemonStat {
                    stat: StatName::Hp,
                    effort: 0,
                    base_stat: 100,
                },
                PokemonStat {
                    stat: StatName::Attack,
                    effort: 0,
                    base_stat: 50,
                },
                PokemonStat {
                    stat: StatName::Defense,
                    effort: 0,
                    base_stat: 40,
                },
                PokemonStat {
                    stat: StatName::SpecialAttack,
                    effort: 0,
                    base_stat: 60,
                },
                PokemonStat {
                    stat: StatName::SpecialDefense,
                    effort: 0,
                    base_stat: 55,
                },
                PokemonStat {
                    stat: StatName::Speed,
                    effort: 0,
                    base_stat: 70,
                },
            ],
            types: vec![],
        }
    }

    fn create_test_ability() -> Ability {
        Ability {
            id: 1,
            name: "Test-Ability".to_string(),
            effect_entries: vec![],
            flavor_text_entries: vec![],
            names: vec![],
        }
    }

    #[test]
    fn test_new_active_pokemon() {
        let pokemon = create_test_pokemon();
        let ability = create_test_ability();
        let active_pokemon = ActivePokemon::new(&pokemon, &ability);

        assert_eq!(active_pokemon.name, "Testmon");
        assert_eq!(active_pokemon.max_hp, 100);
        assert_eq!(active_pokemon.current_hp, 100);
        assert_eq!(active_pokemon.attack, 50);
        assert_eq!(active_pokemon.speed, 70);
        assert_eq!(active_pokemon.attack_rank, 0);
        assert!(active_pokemon.status.is_none());
        assert_eq!(active_pokemon.active_ability.name, "Test-Ability");
    }

    #[test]
    fn test_take_damage() {
        let pokemon = create_test_pokemon();
        let ability = create_test_ability();
        let mut active_pokemon = ActivePokemon::new(&pokemon, &ability);

        active_pokemon.take_damage(30);
        assert_eq!(active_pokemon.current_hp, 70);
        assert!(!active_pokemon.is_fainted());
    }

    #[test]
    fn test_take_fatal_damage() {
        let pokemon = create_test_pokemon();
        let ability = create_test_ability();
        let mut active_pokemon = ActivePokemon::new(&pokemon, &ability);

        active_pokemon.take_damage(120);
        assert_eq!(active_pokemon.current_hp, 0);
        assert!(active_pokemon.is_fainted());
    }

    #[test]
    fn test_set_status() {
        let pokemon = create_test_pokemon();
        let ability = create_test_ability();
        let mut active_pokemon = ActivePokemon::new(&pokemon, &ability);

        assert!(active_pokemon.status.is_none());
        active_pokemon.set_status(StatusCondition::Poison);
        assert_eq!(active_pokemon.status, Some(StatusCondition::Poison));
    }
}
