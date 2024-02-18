use yew::prelude::*;
use yewflow::{edge::{builtin_edges::{straight_edge::StraightEdge}, edge_model::EdgeModel, edge_view_wrapper::EdgeViewProps}, node::{drag_handle::DragHandle, handle::{Handle, HandleType}, node_model::NodeModel, node_view_wrapper::NodeViewProps}, panel::Panel};

#[derive(PartialEq, Clone)]
pub struct NodeViewData {
    pub source_count: usize,
    pub target_count: usize
}

#[function_component(NodeView)]
pub fn node_view(props: &NodeViewProps<NodeViewData>) -> Html {

    let NodeViewProps { node } = props;

    html! {
        <div class={"node-view"}>
            <DragHandle class={"drag-handle-node-view"}>
                <div class={"node-view-content"}>
                    <div class={"node-view-handles"}>
                        {(0..node.data.target_count).map(|i| {
                            html! {
                                <Handle
                                    key={i}
                                    id={format!("{}{}_target", node.id.clone(), i)}
                                    handle_type={HandleType::Target}
                                    style={"width: 10px; height: 10px; border-radius: 1000px; background-color: red;"}
                                    is_connectable={i % 2 == 0}
                                />
                            }
                        }).collect::<Vec<Html>>()}
                    </div>
                    <div class={"node-view-handles"}>
                        {(0..node.data.source_count).map(|i| {
                            html! {
                                <Handle
                                    key={i}
                                    id={format!("{}{}_source", node.id.clone(), i)}
                                    handle_type={HandleType::Source}
                                    style={"width: 10px; height: 10px; border-radius: 1000px; background-color: blue;"}
                                    is_connectable={i % 2 == 1}
                                />
                            }
                        }).collect::<Vec<Html>>()}
                    </div>
                </div>
            </DragHandle>
        </div>
    }

}

#[derive(Properties, PartialEq, Clone)]
pub struct EdgeViewData {
    pub label: String
}

#[function_component(EdgeView)]
pub fn edge_view(props: &EdgeViewProps<EdgeViewData>) -> Html {

    html! {
        <StraightEdge
            edge_coordinates={props.edge_coordinates.clone()}
            stroke={"orange"}
        />
    }

}

#[function_component(App)]
pub fn app() -> Html {

    let nodes = use_state(|| vec![
        NodeModel{ id: String::from("0"), position: (0.0, 0.0), data: NodeViewData {
            source_count: 1,
            target_count: 2
        } },
        NodeModel{ id: String::from("1"), position: (400.0, 100.0), data: NodeViewData {
            source_count: 3,
            target_count: 4
        } },
        NodeModel{ id: String::from("2"), position: (700.0, 100.0), data: NodeViewData {
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
            data: EdgeViewData {
                label: String::from("Hello")
            }
        }
    ]);

    let set_nodes = {
        let nodes = nodes.clone();
        Callback::from(move |new_nodes: Vec<NodeModel<NodeViewData>>| {
            nodes.set(new_nodes)
        })
    };

    let set_edges = {
        let edges = edges.clone();
        Callback::from(move |new_edges: Vec<EdgeModel<EdgeViewData>>| {
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
                data: EdgeViewData {
                    label: String::from("wow")
                },
            };

            set_edges.emit(edges.iter().chain(vec![new_edge].iter()).cloned().collect());
        })
    };

    html! {
        <Panel<NodeViewData, EdgeViewData>
            nodes={(*nodes).clone()}
            set_nodes={set_nodes}
            edges={(*edges).clone()}
            set_edges={set_edges}
            width="100vw"
            height="100vh"
            node_view={Callback::from(|props| html! { <NodeView ..props /> })}
            edge_view={Callback::from(|props| html! { <EdgeView ..props /> })}
            on_create_edge={on_create_edge}
            style={"background-color: gray; position: relative; overflow: hidden; transform-origin: 0px 0px 0px;"}
        />
    }
    
}

fn main() {
    yew::Renderer::<App>::new().render();
}