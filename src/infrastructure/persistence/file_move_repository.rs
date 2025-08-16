use crate::domain::model::r#move::Move;
use crate::domain::repository::move_repository::MoveRepository;
use crate::infrastructure::persistence::json_loader::load_json_from_directory;

/// スカーレット・バイオレット対応言語
const SUPPORTED_LANGUAGES: &[&str] = &["ja", "en"];
/// スカーレット・バイオレットのバージョングループ
const SCARLET_VIOLET_VERSION_GROUP: &str = "scarlet-violet";

pub struct FileMoveRepository;

impl MoveRepository for FileMoveRepository {
    fn find_all_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = load_json_from_directory("data/move/", "moves");

        // スカーレット・バイオレット向けにフィルタリング
        for move_data in &mut moves {
            move_data.flavor_text_entries.retain(|entry| {
                entry.version_group.name == SCARLET_VIOLET_VERSION_GROUP
            });
        }

        moves
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
