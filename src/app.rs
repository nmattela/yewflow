use yew::prelude::*;

use crate::{panel::Panel, node::node_model::NodeModel, edge::edge_model::EdgeModel};

#[function_component(App)]
pub fn app() -> Html {

    let nodes = use_state(|| vec![
        NodeModel{ id: String::from("0"), position: (100, 100) },
        NodeModel{ id: String::from("1"), position: (400, 100) },
        NodeModel{ id: String::from("2"), position: (700, 100) }
    ]);

    let edges = use_state(|| vec![
        EdgeModel{
            id: String::from("edge_0"),
            start_id: String::from("0"),
            end_id: String::from("1"),
            source_handle_id: String::from("from_0"),
            target_handle_id: String::from("to_1"),
        }
    ]);

    let set_nodes = {
        let nodes = nodes.clone();
        Callback::from(move |new_nodes: Vec<NodeModel>| {
            nodes.set(new_nodes)
        })
    };

    let set_edges = {
        let edges = edges.clone();
        Callback::from(move |new_edges: Vec<EdgeModel>| {
            edges.set(new_edges)
        })
    };

    html! {
        <Panel
            nodes={(*nodes).clone()}
            set_nodes={set_nodes}
            edges={(*edges).clone()}
            set_edges={set_edges}
            height="500px"
        />
    }
}
