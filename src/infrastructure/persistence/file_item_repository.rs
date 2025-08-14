use crate::domain::model::Item;
use crate::domain::repository::item_repository::ItemRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

pub struct FileItemRepository;

impl ItemRepository for FileItemRepository {
    fn find_all_items(&self) -> Vec<Item> {
        load_json_from_directory("data/item/", "items")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_items() {
        let repository = FileItemRepository;
        let items = repository.find_all_items();
        // The data/item directory contains many json files, so it should not be empty.
        assert!(!items.is_empty());
    }
}
