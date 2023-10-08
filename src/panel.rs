
use yew_hooks::{UseMapHandle, use_map};
use std::collections::HashMap;



use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, EventTarget};
use yew::prelude::*;


use crate::node::node_model::NodeModel;
use crate::node::node_view::NodeView;
use crate::edge::edge_model::EdgeModel;
use crate::edge::edge_view::EdgeView;
use crate::utils::Position;

#[derive(Properties, PartialEq)]
pub struct PanelProps {
    pub height: String,
    pub nodes: Vec<NodeModel>,
    pub set_nodes: Callback<Vec<NodeModel>>,
    pub edges: Vec<EdgeModel>,
    pub set_edges: Callback<Vec<EdgeModel>>,
}

#[function_component(Panel)]
pub fn panel(props: &PanelProps) -> Html {

    let panel_ref = use_node_ref();

    let currently_dragged_node = use_state(|| None::<(String, Position)>);

    let preview_edge = use_state(|| None::<EdgeModel>);

    let handle_registry: UseMapHandle<String, Position> = use_map(HashMap::new());

    let set_node: Callback<NodeModel> = {
        let nodes = props.nodes.clone();
        let set_nodes = props.set_nodes.clone();
        Callback::from(move |node: NodeModel| {
            let new_nodes = nodes.iter().map(|n| {
                if n.id == node.id {
                    node.clone()
                } else {
                    n.clone()
                }
            }).collect();
            set_nodes.emit(new_nodes);
        })
    };

    let set_edge: Callback<EdgeModel> = {
        let edges = props.edges.clone();
        let set_edges = props.set_edges.clone();
        Callback::from(move |edge: EdgeModel| {
            let new_edges = edges.iter().map(|e| {
                if e.id == edge.id {
                    edge.clone()
                } else {
                    e.clone()
                }
            }).collect();
            set_edges.emit(new_edges);
        })
    };

    let offset = use_state(|| (0, 0));
    let client = use_state(|| (0, 0));
    let rct = use_state(|| (0, 0));
    let relative = use_state(|| (0, 0));

    let on_mouse_down = {
        let currently_dragged_node = currently_dragged_node.clone();
        let offset = offset.clone();
        let client = client.clone();
        let rct = rct.clone();
        let relative = relative.clone();
        let preview_edge = preview_edge.clone();
        Callback::from(move |e: MouseEvent| {
            let event_target: Option<EventTarget> = e.target();
            let handle: Option<HtmlElement> = event_target.and_then(|t| t.dyn_into().ok());
            handle.map(|handle| {
                let class_names = handle.get_attribute("class").unwrap_or_else(|| String::from(""));

                // Offset of drag handle in relation to node
                let offset_left = handle.offset_left();
                let offset_top = handle.offset_top();
                offset.set((offset_left, offset_top));
                // Absolute position of the mouse (though it is wrong)
                let client_x = e.client_x();
                let client_y = e.client_y();
                client.set((client_x - 8, client_y - 8));
                // Top left absolute position of the drag handle (though it is wrong)
                let rect = handle.get_bounding_client_rect();
                let rect_left = rect.x() as i32;
                let rect_top = rect.y() as i32;
                rct.set((rect_left - 8, rect_top - 8));
                let relative_left = client_x - rect_left + 8;
                let relative_top = client_y - rect_top + 8;

                let position_x = offset_left + relative_left;
                let position_y = offset_top + relative_top;

                if class_names.contains("drag-handle") {
                    handle.parent_element().map(|node| {
                        let class_names = node.get_attribute("class").unwrap_or_else(|| String::from(""));
                        let node_id = node.id();
                        if class_names.contains("node") && !node_id.is_empty() {
                            relative.set((relative_left, relative_top));
                            currently_dragged_node.set(Some((node_id, (position_x, position_y))));
                        };
                        Some(())
                    });
                } else if class_names.contains("source-handle") {
                    preview_edge.set(Some(
                        EdgeModel {
                            id: "preview_edge".to_string(),
                            start_id: handle.id(),
                            source_handle_id: handle.id(),
                            end_id: "preview_handle".to_string(),
                            target_handle_id: "preview_handle".to_string()
                        }
                    ));
                    
                }
                Some(())
            });
        })
    };

    let on_mouse_up = {
        let currently_dragged_node = currently_dragged_node.clone();
        let preview_edge = preview_edge.clone();
        let handle_registry = handle_registry.clone();
        let set_edges = props.set_edges.clone();
        let edges = props.edges.clone();
        Callback::from(move |e: MouseEvent| {
            currently_dragged_node.set(None);
            preview_edge.set(None);
            (*preview_edge).clone().and_then(|edge| e.target().and_then(|target| target.dyn_into().ok().map(|target: HtmlElement| {
                handle_registry.remove(&edge.target_handle_id);
                let class_names = target.get_attribute("class").unwrap_or_else(|| String::from(""));
                if class_names.contains("target-handle") {
                    let new_edge = EdgeModel {
                        target_handle_id: target.id(),
                        ..edge.clone()
                    };
                    set_edges.emit(edges.iter().chain(vec![new_edge].iter()).cloned().collect());
                }
                
            })));
        })
    };

    let on_mouse_move = {
        let nodes = props.nodes.clone();
        let set_node = set_node.clone();
        let panel_ref = panel_ref.clone();
        let preview_edge = preview_edge.clone();
        let handle_registry = handle_registry.clone();
        Callback::from(move |event: MouseEvent| {
            let x = event.client_x();
            let y = event.client_y();
            panel_ref.cast::<HtmlElement>().map(|panel_ref| {
                (*currently_dragged_node).clone().and_then(|(cdn, (offset_x, offset_y))|
                    nodes.iter().find(|node| node.id.eq(&cdn)).map(|node| {
                        let width = panel_ref.client_width();
                        let height = panel_ref.client_height();
                        if x >= 0 && x <= width && y >= 0 && y <= height {
                            let new_node = NodeModel { position: (x - offset_x, y - offset_y), ..node.clone() };
                            set_node.emit(new_node);
                        }
                                                
                    })
                );
                (*preview_edge).clone().and_then(|edge| {
                    handle_registry.insert(edge.target_handle_id, (x - 8, y - 8))
                });
                Some(())
            });  
        })
    };

    html!{
        <div
            ref={panel_ref.clone()}
            style={format!("height: {}; background-color: gray; position: relative; overflow: hidden", props.height)}
            // ondragover={on_mouse_dr1ag}
            onmousedown={on_mouse_down}
            onmouseup={on_mouse_up}
            onmousemove={on_mouse_move}
        >
            {props.edges.iter().map(|edge| {
                let edge = edge.clone();
                let handle_registry = handle_registry.clone();
                let set_edge = set_edge.clone();
                html! {
                    <EdgeView
                        edge={edge}
                        handle_registry={handle_registry}
                        set_edge={set_edge}
                    />
                }
            }).collect::<Html>()}
            {
                match &(*preview_edge) {
                    Some(preview_edge) => {
                        let preview_edge = preview_edge.clone();
                        let handle_registry = handle_registry.clone();
                        let set_edge = set_edge.clone();
                        html! {
                            <EdgeView
                                edge={preview_edge}
                                handle_registry={handle_registry}
                                set_edge={set_edge}
                            />
                        }
                    },
                    None => html! {}
                }
            }
            {props.nodes.iter().enumerate().map(|(_i, node)| {
                let set_node = set_node.clone();
                let node = node.clone();
                let handle_registry = handle_registry.clone();
                html! {
                    <NodeView
                        node={node}
                        set_node={Callback::from(move |node| set_node.emit(node))}
                        handle_registry={handle_registry}
                    />
                }
            }).collect::<Html>()}
        </div>
    }


}