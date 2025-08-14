use crate::domain::model::Move;

pub trait MoveRepository {
    fn find_all_moves(&self) -> Vec<Move>;
}
