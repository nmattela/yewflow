

use gloo_console::{warn};

use yew::prelude::*;
use yew_hooks::UseMapHandle;

use crate::{utils::Position};

use super::edge_model::EdgeModel;

#[derive(Properties, PartialEq)]
pub struct EdgeViewProps {
    pub edge: EdgeModel,

    pub handle_registry: UseMapHandle<String, Position>,
    pub set_edge: Callback<EdgeModel>
}

#[function_component(EdgeView)]
pub fn edge_view(props: &EdgeViewProps) -> Html {

    let EdgeViewProps { edge, handle_registry, set_edge: _ } = props;

    let start_coordinates: Result<Position, String> = {
        let current = handle_registry.current();
        let handle = current.get(&edge.source_handle_id);

        match handle {
            Some(source_handle) => {
                Ok(*source_handle)
            },
            None => Err(format!("edge with ID {} was supposed to connect to source handle ID {} which is a node of ID {}, but that handle does not exist", edge.id, edge.source_handle_id, edge.start_id))
        }
    };

    let end_coordinates: Result<Position, String> = {
        let current = handle_registry.current();
        let handle = current.get(&edge.target_handle_id);

        match handle {
            Some(target_handle) => {
                Ok(*target_handle)
            },
            None => {
                Err(format!("edge with ID {} was supposed to connect to target handle ID {} which is a node of ID {}, but that handle does not exist", edge.id, edge.target_handle_id, edge.end_id))
            }
        }
    };

    match start_coordinates.and_then(|start| end_coordinates.map(|end| (start, end))) {
        Ok((start_coordinates, end_coordinates)) => {
            html! {
                <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg" class="edge">
                    <line x1={start_coordinates.0.to_string()} y1={start_coordinates.1.to_string()} x2={end_coordinates.0.to_string()} y2={end_coordinates.1.to_string()} stroke="black" />
                </svg>
            }
        },
        Err(e) => {
            warn!(e);
            html! {}
        }
    }

}