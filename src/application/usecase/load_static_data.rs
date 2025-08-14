use crate::application::dto::loaded_static_data::LoadedStaticData;
use crate::domain::repository::item_repository::ItemRepository;
use crate::domain::repository::move_repository::MoveRepository;
use crate::domain::repository::pokemon_repository::PokemonRepository;
use crate::domain::repository::pokemon_species_repository::PokemonSpeciesRepository;
use crate::domain::repository::type_repository::TypeRepository;

pub struct LoadStaticDataUsecase<I, P, M, S, T>
where
    I: ItemRepository,
    P: PokemonRepository,
    M: MoveRepository,
    S: PokemonSpeciesRepository,
    T: TypeRepository,
{
    item_repository: I,
    pokemon_repository: P,
    move_repository: M,
    pokemon_species_repository: S,
    type_repository: T,
}

impl<I, P, M, S, T> LoadStaticDataUsecase<I, P, M, S, T>
where
    I: ItemRepository,
    P: PokemonRepository,
    M: MoveRepository,
    S: PokemonSpeciesRepository,
    T: TypeRepository,
{
    pub fn new(
        item_repository: I,
        pokemon_repository: P,
        move_repository: M,
        pokemon_species_repository: S,
        type_repository: T,
    ) -> Self {
        Self {
            item_repository,
            pokemon_repository,
            move_repository,
            pokemon_species_repository,
            type_repository,
        }
    }

    pub fn execute(&self) -> LoadedStaticData {
        let items = self.item_repository.find_all_items();
        let pokemons = self.pokemon_repository.find_all_pokemons();
        let moves = self.move_repository.find_all_moves();
        let pokemon_species = self.pokemon_species_repository.find_all_species();
        let types = self.type_repository.find_all_types();

        LoadedStaticData {
            items,
            pokemons,
            moves,
            pokemon_species,
            types,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::persistence::file_item_repository::FileItemRepository;
    use crate::infrastructure::persistence::file_move_repository::FileMoveRepository;
    use crate::infrastructure::persistence::file_pokemon_repository::FilePokemonRepository;
    use crate::infrastructure::persistence::file_pokemon_species_repository::FilePokemonSpeciesRepository;
    use crate::infrastructure::persistence::file_type_repository::FileTypeRepository;

    #[test]
    fn test_load_static_data_usecase() {
        let item_repository = FileItemRepository;
        let pokemon_repository = FilePokemonRepository;
        let move_repository = FileMoveRepository;
        let pokemon_species_repository = FilePokemonSpeciesRepository;
        let type_repository = FileTypeRepository;

        let usecase = LoadStaticDataUsecase::new(
            item_repository,
            pokemon_repository,
            move_repository,
            pokemon_species_repository,
            type_repository,
        );

        let loaded_data = usecase.execute();

        assert!(!loaded_data.items.is_empty());
        assert!(!loaded_data.pokemons.is_empty());
        assert!(!loaded_data.moves.is_empty());
        assert!(!loaded_data.pokemon_species.is_empty());
        assert!(!loaded_data.types.is_empty());
    }
}
