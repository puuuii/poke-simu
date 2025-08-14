use crate::domain::model::Item;

pub trait ItemRepository {
    fn find_all_items(&self) -> Vec<Item>;
}
