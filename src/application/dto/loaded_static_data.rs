use crate::domain::model::{Item, Pokemon, Move, pokemon_species::PokemonSpecies, poke_type::PokeType};

pub struct LoadedStaticData {
    pub items: Vec<Item>,
    pub pokemons: Vec<Pokemon>,
    pub moves: Vec<Move>,
    pub pokemon_species: Vec<PokemonSpecies>,
    pub types: Vec<PokeType>,
}
