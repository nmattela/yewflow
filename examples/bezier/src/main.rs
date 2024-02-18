use yew::prelude::*;
use yewflow::{edge::{builtin_edges::bezier_edge::BezierEdge, edge_model::EdgeModel, edge_view_wrapper::EdgeViewProps}, node::node_model::NodeModel, panel::Panel};

#[function_component(EdgeView)]
pub fn edge_view(props: &EdgeViewProps<()>) -> Html {
    html! {
        <>
            <marker
                id="head"
                viewBox="-10 0 10 10"
                refX="1"
                refY="5"
                markerUnits="strokeWidth"
                markerWidth="10"
                markerHeight="10"
                orient="auto"
            >
                <path d="M -10 0 L 0 5 L -10 10 z" fill="#000" />
            </marker>
            <BezierEdge
                edge_coordinates={props.edge_coordinates.clone()}
                stroke={"black"}
                marker_end={"url(#head)"}
            />
        </>
    }
}

#[function_component(App)]
pub fn app() -> Html {

    let nodes = use_state(|| vec![
        NodeModel {
            id: String::from("0"),
            position: (0.0, 0.0),
            data: (),
        },
        NodeModel {
            id: String::from("1"),
            position: (200.0, 200.0),
            data: (),
        },
    ]);

    let edges = use_state(|| vec![
        EdgeModel {
            id: String::from("edge_0"),
            start_id: String::from("0"),
            end_id: String::from("1"),
            source_handle_id: String::from("0_source"),
            target_handle_id: String::from("1_target"),
            data: (),
        }
    ]);

    let set_nodes = {
        let nodes = nodes.clone();
        Callback::from(move |new_nodes: Vec<NodeModel<()>>| {
            nodes.set(new_nodes)
        })
    };

    let set_edges = {
        let edges = edges.clone();
        Callback::from(move |new_edges: Vec<EdgeModel<()>>| {
            edges.set(new_edges)
        })
    };

    html! {
        <Panel
            nodes={(*nodes).clone()}
            set_nodes={set_nodes}
            edges={(*edges).clone()}
            set_edges={set_edges}
            width="100vw"
            height="100vh"
            edge_view={Callback::from(|props| html! { <EdgeView ..props /> })}
        />
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
