use crate::domain::model::pokemon_species::PokemonSpecies;

pub trait PokemonSpeciesRepository {
    fn find_all_species(&self) -> Vec<PokemonSpecies>;
}
