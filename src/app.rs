use yew::prelude::*;

use crate::{panel::Panel, node::{node_model::NodeModel, node_view2::{NodeView2, NodeView2Data}}, edge::{edge_model::EdgeModel, edge_view2::{EdgeView2Data, EdgeView2}}};

#[function_component(App)]
pub fn app() -> Html {

    let nodes = use_state(|| vec![
        NodeModel{ id: String::from("0"), position: (0.0, 0.0), data: NodeView2Data {
            source_count: 1,
            target_count: 2
        } },
        NodeModel{ id: String::from("1"), position: (400.0, 100.0), data: NodeView2Data {
            source_count: 3,
            target_count: 4
        } },
        NodeModel{ id: String::from("2"), position: (700.0, 100.0), data: NodeView2Data {
            source_count: 5,
            target_count: 6
        } }
    ]);

    let edges = use_state(|| vec![
        EdgeModel{
            id: String::from("edge_0"),
            start_id: String::from("0"),
            end_id: String::from("1"),
            source_handle_id: String::from("00_source"),
            target_handle_id: String::from("10_target"),
            data: EdgeView2Data {
                label: String::from("Hello")                
            }
        }
    ]);

    let set_nodes = {
        let nodes = nodes.clone();
        Callback::from(move |new_nodes: Vec<NodeModel<NodeView2Data>>| {
            nodes.set(new_nodes)
        })
    };

    let set_edges = {
        let edges = edges.clone();
        Callback::from(move |new_edges: Vec<EdgeModel<EdgeView2Data>>| {
            edges.set(new_edges)
        })
    };

    let on_create_edge = {
        let edges = edges.clone();
        let set_edges = set_edges.clone();
        Callback::from(move |edge: EdgeModel<()>| {
            let new_edge = EdgeModel {
                id: edge.id,
                start_id: edge.start_id,
                end_id: edge.end_id,
                source_handle_id: edge.source_handle_id,
                target_handle_id: edge.target_handle_id,
                data: EdgeView2Data {
                    label: String::from("wow")
                },
            };

            set_edges.emit(edges.iter().chain(vec![new_edge].iter()).cloned().collect());
        })
    };

    html! {
        <Panel<NodeView2Data, EdgeView2Data>
            nodes={(*nodes).clone()}
            set_nodes={set_nodes}
            edges={(*edges).clone()}
            set_edges={set_edges}
            width="100vw"
            height="100vh"
            node_view={Callback::from(|props| html! { <NodeView2 ..props /> })}
            edge_view={Callback::from(|props| html! { <EdgeView2 ..props /> })}
            on_create_edge={on_create_edge}
        />
    }
}
