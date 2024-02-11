use crate::utils::Position;

#[derive(PartialEq, Clone)]
pub struct NodeModel<T: PartialEq + Clone> {
    pub id: String,
    pub position: Position,
    pub data: T
}