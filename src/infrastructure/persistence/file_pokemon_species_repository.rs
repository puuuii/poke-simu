use crate::domain::model::pokemon_species::PokemonSpecies;
use crate::domain::repository::pokemon_species_repository::PokemonSpeciesRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

const SCARLET_VIOLET_VERSIONS: [&str; 2] = ["scarlet", "violet"];

pub struct FilePokemonSpeciesRepository;

impl PokemonSpeciesRepository for FilePokemonSpeciesRepository {
    fn find_all_species(&self) -> Vec<PokemonSpecies> {
        let mut species_list: Vec<PokemonSpecies> = load_json_from_directory("data/pokemon-species/", "pokemon-species");
        
        // Filter flavor text entries to only include Scarlet-Violet data
        for species in &mut species_list {
            species.flavor_text_entries.retain(|entry| {
                SCARLET_VIOLET_VERSIONS.contains(&entry.version.name.as_str())
            });
        }
        
        species_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_species() {
        let repository = FilePokemonSpeciesRepository;
        let species = repository.find_all_species();
        // The data/pokemon-species directory contains many json files, so it should not be empty.
        assert!(!species.is_empty());
    }
}
