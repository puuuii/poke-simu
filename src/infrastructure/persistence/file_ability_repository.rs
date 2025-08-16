use crate::domain::model::ability::Ability;
use crate::domain::repository::ability_repository::AbilityRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

/// スカーレット・バイオレット対応言語
const SUPPORTED_LANGUAGES: &[&str] = &["ja", "en"];
/// スカーレット・バイオレットのバージョングループ
const SCARLET_VIOLET_VERSION_GROUP: &str = "scarlet-violet";

pub struct FileAbilityRepository;

impl AbilityRepository for FileAbilityRepository {
    fn find_all_abilities(&self) -> Vec<Ability> {
        let mut abilities: Vec<Ability> = load_json_from_directory("data/ability/", "abilities");

        // スカーレット・バイオレット向けにフィルタリング
        for ability in &mut abilities {
            ability.flavor_text_entries.retain(|entry| {
                entry.version_group.name == SCARLET_VIOLET_VERSION_GROUP
                    && SUPPORTED_LANGUAGES.contains(&entry.language.name.as_str())
            });
        }

        abilities
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_abilities() {
        let repository = FileAbilityRepository;
        let abilities = repository.find_all_abilities();
        // The data/ability directory contains many json files, so it should not be empty.
        assert!(!abilities.is_empty());
    }
}
