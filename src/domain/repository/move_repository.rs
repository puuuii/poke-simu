use crate::domain::model::r#move::Move;

pub trait MoveRepository {
    fn find_all_moves(&self) -> Vec<Move>;
}
