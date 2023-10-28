








use gloo_console::log;
use yew_hooks::{UseMapHandle, use_map};

use std::collections::HashMap;



use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, EventTarget, DomRect};
use yew::prelude::*;


use crate::edge::default_preview_edge_view::DefaultPreviewEdgeView;
use crate::hooks::use_register_handles::use_register_handles;
use crate::node::node_model::NodeModel;
use crate::edge::edge_model::EdgeModel;
use crate::edge::edge_view_wrapper::{EdgeViewWrapper, EdgeViewProps};
use crate::node::node_view_wrapper::{NodeViewWrapper, NodeViewProps};
use crate::utils::{Position, AttributeExtractHelper};

#[derive(Properties, PartialEq)]
pub struct PanelProps<NodeData: PartialEq + Clone, EdgeData: PartialEq + Clone> {
    pub width: String,
    pub height: String,
    pub nodes: Vec<NodeModel<NodeData>>,
    pub set_nodes: Callback<Vec<NodeModel<NodeData>>>,
    pub edges: Vec<EdgeModel<EdgeData>>,
    pub set_edges: Callback<Vec<EdgeModel<EdgeData>>>,

    pub on_create_edge: Callback<EdgeModel<()>>,

    pub node_view: Callback<NodeViewProps<NodeData>, Html>,
    pub edge_view: Callback<EdgeViewProps<EdgeData>, Html>,
    #[prop_or_default]
    pub preview_edge_view: Option<Callback<EdgeViewProps<()>, Html>>,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    old_x: f64,
    old_y: f64
}

impl Viewport {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Viewport {
            x,
            y,
            z,
            old_x: x,
            old_y: y,
        }
    }

    pub fn pan_start(&self, (x, y): Position) -> Self {
        Viewport {
            x: self.x,
            y: self.y,
            z: self.z,
            old_x: x,
            old_y: y,
        }
    }

    pub fn pan(&self, (x, y): Position) -> Self {

        let base_x = (x - self.old_x) * self.z;
        let base_y = (y - self.old_y) * self.z;

        let _transformed_x = {
            let powed = base_x.abs().powf(self.z);
            if base_x >= 0.0 {
                powed
            } else {
                powed * -1.0
            }
        };

        let _transformed_y = {
            let powed = base_y.abs().powf(self.z);
            if base_y >= 0.0 {
                powed
            } else {
                powed * -1.0
            }
        };

        Viewport {
            x: self.x + (x - self.old_x),
            y: self.y + (y - self.old_y),
            z: self.z,
            old_x: x,
            old_y: y
        }
    }

    pub fn zoom(&self, container_rect: DomRect, (_x, _y, z): (f64, f64, f64), (mouse_x, mouse_y): (f64, f64)) -> Self {
        let new_z = self.z + z;
        if !(0.5..=2.0).contains(&new_z) {
            *self
        } else {
            Viewport {
                x: self.x + ((container_rect.width() * (z / 2.0)) / self.z) + (((mouse_x - self.x) * self.z - (mouse_x - self.x) * new_z) / self.z),
                y: self.y + (((mouse_y - self.y) * self.z - (mouse_y - self.y) * new_z) / self.z),
                z: new_z,
                old_x: self.old_x,
                old_y: self.old_y
            }
        }

    }
}

#[function_component(Panel)]
pub fn panel<NodeData: PartialEq + Clone + 'static, EdgeData: PartialEq + Clone + 'static>(props: &PanelProps<NodeData, EdgeData>) -> Html {

    let panel_ref = use_node_ref();
    let nodes_ref = use_node_ref();

    let currently_dragged_node = use_state(|| None::<(String, Position)>);
    let preview_edge = use_state(|| None::<EdgeModel<()>>);
    let handle_registry: UseMapHandle<String, Position> = use_map(HashMap::new());
    let viewport: UseStateHandle<Viewport> = use_state(|| Viewport::new(0.0, 0.0, 1.0));
    let panning: UseStateHandle<bool> = use_state(|| false);

    handle_registry.clone().current().iter().for_each(|(key, value)| {
        log!(key, value.0, value.1);
    });

    use_register_handles(nodes_ref.clone(), handle_registry.clone(), *viewport.clone());

    let set_node: Callback<NodeModel<NodeData>> = {
        let nodes = props.nodes.clone();
        let set_nodes = props.set_nodes.clone();
        Callback::from(move |node: NodeModel<NodeData>| {
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

    let set_edge: Callback<EdgeModel<EdgeData>> = {
        let edges = props.edges.clone();
        let set_edges = props.set_edges.clone();
        Callback::from(move |edge: EdgeModel<EdgeData>| {
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

    let on_mouse_down = {
        let currently_dragged_node = currently_dragged_node.clone();
        let preview_edge = preview_edge.clone();
        let viewport = viewport.clone();
        let panning = panning.clone();
        Callback::from(move |e: MouseEvent| {
            let event_target: Option<EventTarget> = e.target();
            let handle: Option<HtmlElement> = event_target.and_then(|t| t.dyn_into().ok());
            handle.map(|handle| {
                let _class_names = handle.get_class_names();

                // Offset of drag handle in relation to node
                let offset_left = handle.offset_left() as f64;
                let offset_top = handle.offset_top() as f64;
                // Absolute position of the mouse (though it is wrong)
                let client_x = e.client_x() as f64;
                let client_y = e.client_y() as f64;
                // Top left absolute position of the drag handle (though it is wrong)
                let rect = handle.get_bounding_client_rect();
                let rect_left = rect.x();
                let rect_top = rect.y();
                let relative_left = client_x - rect_left;// + 8.0;
                let relative_top = client_y - rect_top;// + 8.0;

                let position_x = offset_left + relative_left;
                let position_y = offset_top + relative_top;

                if let Some(source_handle) = handle.parent_element_with_class("source-handle".to_string()) {
                    preview_edge.set(Some(
                        EdgeModel {
                            id: "preview_edge".to_string(),
                            start_id: handle.id(),
                            source_handle_id: source_handle.id(),
                            end_id: "preview_handle".to_string(),
                            target_handle_id: "preview_handle".to_string(),
                            data: (),
                        }
                    ));
                } else if let Some(drag_handle) = handle.parent_element_with_class("drag-handle".to_string()) {
                    let node = drag_handle.parent_element_with_class("node".to_string());
                    if let Some(node) = node {
                        currently_dragged_node.set(Some((node.id(), (position_x, position_y))));
                    }
                } else {
                    panning.set(true);
                    viewport.set(viewport.pan_start((client_x, client_y)));
                }
                Some(())
            });
        })
    };

    let on_mouse_up = {
        let currently_dragged_node = currently_dragged_node.clone();
        let preview_edge = preview_edge.clone();
        let handle_registry = handle_registry.clone();
        let _set_edges = props.set_edges.clone();
        let _edges = props.edges.clone();
        let panning = panning.clone();
        let on_create_edge = props.on_create_edge.clone();
        Callback::from(move |e: MouseEvent| {
            currently_dragged_node.set(None);
            preview_edge.set(None);
            panning.set(false);
            (*preview_edge).clone().and_then(|preview_edge| e.target().and_then(|target| target.dyn_into().ok().map(|target: HtmlElement| {
                handle_registry.remove(&preview_edge.target_handle_id);
                let class_names = target.get_class_names();
                if class_names.contains(&"target-handle".to_string()) {
                    on_create_edge.emit(EdgeModel {
                        target_handle_id: target.id(),
                        ..preview_edge.clone()
                    });
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
        let viewport = viewport.clone();
        let panning = panning.clone();
        let currently_dragged_node = currently_dragged_node.clone();
        Callback::from(move |event: MouseEvent| {
            let x = event.client_x() as f64;
            let y = event.client_y() as f64;

            if *panning {
                viewport.set(viewport.pan((
                    x,
                    y
                )))
            }

            panel_ref.cast::<HtmlElement>().map(|panel_ref| {
                (*currently_dragged_node).clone().and_then(|(cdn, (offset_x, offset_y))|
                    nodes.iter().find(|node| node.id.eq(&cdn)).map(|node| {
                        let width = panel_ref.client_width() as f64;
                        let height = panel_ref.client_height() as f64;
                        if x >= 0.0 && x <= width && y >= 0.0 && y <= height {
                            let new_node = NodeModel {
                                position: (
                                    (x / viewport.z) + ((width * viewport.z - width) / (viewport.z * 2.0)) - (viewport.x / viewport.z) - (offset_x / viewport.z),
                                    (y / viewport.z) - (viewport.y / viewport.z) - (offset_y / viewport.z)
                                ),
                                ..node.clone()
                            };
                            set_node.emit(new_node);
                        }
                                                
                    })
                );
                (*preview_edge).clone().and_then(|edge| {
                    handle_registry.insert(edge.target_handle_id, (x, y))
                });
                Some(())
            });  
        })
    };

    let on_wheel = {
        let viewport = viewport.clone();
        let panel_ref = panel_ref.clone();
        Callback::from(move |event: WheelEvent| {
            let sign = if event.delta_y() > 0.0 { -1.0 } else { 1.0 };
            let _ = panel_ref.cast::<HtmlElement>().map(|element| {
                let rect = element.get_bounding_client_rect();
                let x = event.clone().client_x() as f64;
                let y = event.clone().client_y() as f64;
                let z = 0.1 * sign;
                viewport.set(viewport.zoom(rect, (x, y, z), (event.client_x() as f64, event.client_y() as f64)));
            });
        })
    };

    html!{
        <div
            ref={panel_ref.clone()}
            style={format!("height: {}; background-color: gray; position: relative; overflow: hidden; transform-origin: 0px 0px 0px;", props.height)}
            onmousedown={on_mouse_down}
            onmouseup={on_mouse_up}
            onmousemove={on_mouse_move}
            onwheel={on_wheel}
        >
            <div
                style={format!(
                    "position: relative; transform: translate({}px, {}px) scale({});",
                    viewport.x,
                    viewport.y,
                    viewport.z,
                )}
            >
                <div>
                    {props.edges.iter().map(|edge| {
                        let edge = edge.clone();
                        let handle_registry = handle_registry.clone();
                        let set_edge = set_edge.clone();
                        let edge_view = props.edge_view.clone();
                        let panel_ref = panel_ref.clone();
                        html! {
                            <EdgeViewWrapper<EdgeData>
                                edge={edge}
                                panel_ref={panel_ref}
                                handle_registry={handle_registry}
                                viewport={*viewport}
                                set_edge={set_edge}
                                edge_view={edge_view}
                            />
                        }
                    }).collect::<Html>()}
                </div>
                {
                    match &(*preview_edge) {
                        Some(preview_edge) => {
                            let preview_edge = preview_edge.clone();
                            let handle_registry = handle_registry.clone();
                            let panel_ref = panel_ref.clone();
                            let preview_edge_view = match &props.preview_edge_view {
                                Some(preview_edge_view) => preview_edge_view.clone(),
                                None => Callback::from(|props: EdgeViewProps<()>| html! {
                                    <DefaultPreviewEdgeView ..props />
                                })
                            };
                            html! {
                                <EdgeViewWrapper<()>
                                    edge={preview_edge}
                                    panel_ref={panel_ref}
                                    handle_registry={handle_registry}
                                    viewport={*viewport}
                                    set_edge={Callback::from(|_| {})}
                                    edge_view={preview_edge_view}
                                />
                            }
                        },
                        None => html! {}
                    }
                }
                <div ref={nodes_ref}>
                    {props.nodes.iter().enumerate().map(|(_i, node)| {
                        let node = node.clone();
                        let handle_registry = handle_registry.clone();
                        let node_view = props.node_view.clone();
                        html! {
                            <NodeViewWrapper<NodeData>
                                node={node}
                                handle_registry={handle_registry}
                                node_view={node_view}
                            />
                        }
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
    }


}