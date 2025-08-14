use crate::domain::model::Pokemon;

pub trait PokemonRepository {
    fn find_all_pokemons(&self) -> Vec<Pokemon>;
}
