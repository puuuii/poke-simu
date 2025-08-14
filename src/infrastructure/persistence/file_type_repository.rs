use crate::domain::model::poke_type::PokeType;
use crate::domain::repository::type_repository::TypeRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

pub struct FileTypeRepository;

impl TypeRepository for FileTypeRepository {
    fn find_all_types(&self) -> Vec<PokeType> {
        load_json_from_directory("data/type/", "types")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_types() {
        let repository = FileTypeRepository;
        let types = repository.find_all_types();
        // The data/type directory contains many json files, so it should not be empty.
        assert!(!types.is_empty());
    }
}
