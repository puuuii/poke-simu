use crate::domain::model::item::Item;

pub trait ItemRepository {
    fn find_all_items(&self) -> Vec<Item>;
}
