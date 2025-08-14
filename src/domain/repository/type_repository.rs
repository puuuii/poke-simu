use crate::domain::model::poke_type::PokeType;

pub trait TypeRepository {
    fn find_all_types(&self) -> Vec<PokeType>;
}
