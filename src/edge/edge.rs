#[derive(PartialEq, Clone)]
pub struct Edge {
    pub id: String,
    pub start_id: String,
    pub end_id: String,
    pub source_handle_id: String,
    pub target_handle_id: String,
}