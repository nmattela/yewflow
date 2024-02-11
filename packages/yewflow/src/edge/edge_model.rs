/**
 * Contains all the information for a node. The data field is custom data that is provided by the user
 */
#[derive(PartialEq, Clone)]
pub struct EdgeModel<T: PartialEq + Clone> {
    pub id: String,
    pub start_id: String,
    pub end_id: String,
    pub source_handle_id: String,
    pub target_handle_id: String,
    pub data: T
}