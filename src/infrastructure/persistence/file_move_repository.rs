use crate::domain::model::Move;
use crate::domain::repository::move_repository::MoveRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

pub struct FileMoveRepository;

impl MoveRepository for FileMoveRepository {
    fn find_all_moves(&self) -> Vec<Move> {
        load_json_from_directory("data/move/", "moves")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_moves() {
        let repository = FileMoveRepository;
        let moves = repository.find_all_moves();
        // The data/move directory contains many json files, so it should not be empty.
        assert!(!moves.is_empty());
    }
}
