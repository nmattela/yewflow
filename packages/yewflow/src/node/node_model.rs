use crate::utils::Position;

/**
 * Contains all the information for a node. The data field is custom data that is provided by the user.
 */
#[derive(PartialEq, Clone)]
pub struct NodeModel<T: PartialEq + Clone> {
    pub id: String,
    pub position: Position,
    pub data: T
}