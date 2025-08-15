use crate::domain::model::ability::Ability;

pub trait AbilityRepository {
    fn find_all_abilities(&self) -> Vec<Ability>;
}
