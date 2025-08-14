use crate::domain::model::Pokemon;
use crate::domain::repository::pokemon_repository::PokemonRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

pub struct FilePokemonRepository;

impl PokemonRepository for FilePokemonRepository {
    fn find_all_pokemons(&self) -> Vec<Pokemon> {
        load_json_from_directory("data/pokemon/", "pokemons")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_pokemons() {
        let repository = FilePokemonRepository;
        let pokemons = repository.find_all_pokemons();
        // The data/pokemon directory contains many json files, so it should not be empty.
        assert!(!pokemons.is_empty());
    }
}
