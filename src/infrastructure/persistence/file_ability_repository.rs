use crate::domain::model::ability::Ability;
use crate::domain::repository::ability_repository::AbilityRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

pub struct FileAbilityRepository;

impl AbilityRepository for FileAbilityRepository {
    fn find_all_abilities(&self) -> Vec<Ability> {
        load_json_from_directory("data/ability/", "abilities")
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
