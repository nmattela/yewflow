use yew::prelude::*;
use yewflow::{edge::{edge_model::EdgeModel, edge_view_wrapper::{EdgeCoordinates, EdgeViewProps}}, node::{drag_handle::DragHandle, handle::{Handle, HandleType}, node_model::NodeModel, node_view_wrapper::NodeViewProps}, panel::Panel};

#[derive(PartialEq, Clone)]
pub struct NodeViewData {
    pub source_count: usize,
    pub target_count: usize
}

#[function_component(NodeView)]
pub fn node_view(props: &NodeViewProps<NodeViewData>) -> Html {

    let NodeViewProps { node, node_ref } = props;

    html! {
        <div
            ref={node_ref}
        >
            <div class={"node-view-2"}>    
                <DragHandle class={"drag-handle-node-view-2"}>
                    <div class={"node-view-2-content"}>
                        <div class={"node-view-2-handles"}>
                            {(0..node.data.target_count).map(|i| {
                                html! {
                                    <Handle
                                        key={i}
                                        id={format!("{}{}_target", node.id.clone(), i)}
                                        handle_type={HandleType::Target}
                                        style={"background-color: red;"}
                                    />
                                }
                            }).collect::<Vec<Html>>()}
                        </div>
                        // <div>
                        //     {format!("({}, {})", node.position.0, node.position.1)}
                        // </div>
                        <div class={"node-view-2-handles"}>
                            {(0..node.data.source_count).map(|i| {
                                html! {
                                    <Handle
                                        key={i}
                                        id={format!("{}{}_source", node.id.clone(), i)}
                                        handle_type={HandleType::Source}
                                        style={"background-color: blue;"}
                                    />
                                }
                            }).collect::<Vec<Html>>()}
                        </div>
                    </div>
                </DragHandle>
            </div>
        </div>
    }

}

#[derive(Properties, PartialEq, Clone)]
pub struct EdgeViewData {
    pub label: String
}

#[function_component(EdgeView)]
pub fn edge_view(props: &EdgeViewProps<EdgeViewData>) -> Html {

    let EdgeViewProps { edge: _, edge_coordinates: EdgeCoordinates { start_coordinates, end_coordinates } } = props;

    html! {
        <line
            x1={(start_coordinates.0).to_string()}
            y1={(start_coordinates.1).to_string()}
            x2={(end_coordinates.0).to_string()}
            y2={(end_coordinates.1).to_string()}
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
        />
    }
    
}

fn main() {
    yew::Renderer::<App>::new().render();
}