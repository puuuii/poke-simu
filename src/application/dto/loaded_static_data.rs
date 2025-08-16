use crate::domain::model::{
    ability::Ability, item::Item, r#move::Move, poke_type::PokeType, pokemon::Pokemon,
    pokemon_species::PokemonSpecies,
};

pub struct LoadedStaticData {
    pub items: Vec<Item>,
    pub pokemons: Vec<Pokemon>,
    pub moves: Vec<Move>,
    pub pokemon_species: Vec<PokemonSpecies>,
    pub types: Vec<PokeType>,
    pub abilities: Vec<Ability>,
}
