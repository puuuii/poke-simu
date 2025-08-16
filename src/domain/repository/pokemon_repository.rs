use crate::domain::model::pokemon::Pokemon;

pub trait PokemonRepository {
    fn find_all_pokemons(&self) -> Vec<Pokemon>;
}
