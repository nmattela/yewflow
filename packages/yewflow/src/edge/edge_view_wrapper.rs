use gloo_console::warn;

use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::UseMapHandle;

use crate::{hooks::use_register_handles::Handle, panel::Viewport, utils::{AttributeExtractHelper, Position}};

use super::edge_model::EdgeModel;

#[derive(Properties, PartialEq)]
pub struct EdgeViewWrapperProps<T: PartialEq + Clone> {
    pub edge: EdgeModel<T>,
    pub panel_ref: NodeRef,

    pub handle_registry: UseMapHandle<String, Handle>,
    pub viewport: Viewport,
    // Do not remove. Somehow if you do not add it, start and end coordinates do not get updated!
    pub set_edge: Callback<EdgeModel<T>>,

    pub edge_view: Callback<EdgeViewProps<T>, Html>
}

#[derive(Properties, PartialEq, Clone)]
pub struct EdgeCoordinates {
    pub start_coordinates: Position,
    pub end_coordinates: Position,
}

#[derive(Properties, PartialEq)]
pub struct EdgeViewProps<T: PartialEq + Clone> {
    pub edge: EdgeModel<T>,
    pub edge_coordinates: EdgeCoordinates,
}

/**
 * A wrapping component around a provided edge component. It mostly handles the placement of the edge.
 */
#[function_component(EdgeViewWrapper)]
pub fn edge_view_wrapper<T: PartialEq + Clone + 'static>(props: &EdgeViewWrapperProps<T>) -> Html {

    let EdgeViewWrapperProps { edge, panel_ref, handle_registry, viewport, edge_view, set_edge: _ } = props;

    let edge_ref = use_node_ref();

    /*Start coorinates are the position of the source handle */
    let start_coordinates = use_memo(((*handle_registry.current()).clone(), edge.clone(), panel_ref.clone(), *viewport), |(handle_registry, edge, panel_ref, viewport)| {
        /*Find the source handle in the registry */
        let handle = handle_registry.get(&edge.source_handle_id);
        /*Get the width of the panel */
        let width = panel_ref.cast::<HtmlElement>().map(|element| element.get_bounding_client_rect().width());

        match handle.zip(width) {
            Some((Handle { position: (x, y), is_connectable: _ }, width)) => {
                Ok((
                    ((*x - viewport.x) / viewport.z) + ((width * viewport.z - width) / (viewport.z * 2.0)),
                    (*y - viewport.y) / viewport.z
                ))
            },
            None => Err(format!("edge with ID {} was supposed to connect to source handle ID {} which is a handle of node ID {}, but that handle does not exist", edge.id, edge.source_handle_id, edge.start_id))
        }
    });

    /*End coordinates are the position of the target handle */
    let end_coordinates = use_memo(((*handle_registry.current()).clone(), edge.clone(), panel_ref.clone(), *viewport), |(handle_registry, edge, panel_ref, viewport)| {
        /*Find the target handle in the registry */
        let handle = handle_registry.get(&edge.target_handle_id);
        /*Get the width of the panel */
        let width = panel_ref.cast::<HtmlElement>().map(|element| element.get_bounding_client_rect().width());

        match handle.zip(width) {
            Some((Handle { position: (x, y), is_connectable: _ }, width)) => {
                Ok((
                    ((*x - viewport.x) / viewport.z) + ((width * viewport.z - width) / (viewport.z * 2.0)),
                    (*y - viewport.y) / viewport.z
                ))
            },
            None => {
                Err(format!("edge with ID {} was supposed to connect to target handle ID {} which is a handle of node ID {}, but that handle does not exist", edge.id, edge.target_handle_id, edge.end_id))
            }
        }
    });

    /*The box in which the SVG of the edge will be contained in */
    let view_box = use_memo(((*start_coordinates).clone(), (*end_coordinates).clone()), |(start_coordinates, end_coordinates)| {
        start_coordinates.clone().and_then(|start_coordinates| end_coordinates.clone().map(|end_coordinates| {
            let left = start_coordinates.0.min(end_coordinates.0);
            let top = start_coordinates.1.min(end_coordinates.1);
            let width = (start_coordinates.0 - end_coordinates.0).abs();
            let height = (start_coordinates.1 - end_coordinates.1).abs();
            (left, top, width, height)
        }))
    });

    {
        let edge_ref = edge_ref.clone();
        use_effect(move || {
            edge_ref.cast::<HtmlElement>().map(|element| {
                let existing_class = element.get_class_names();
                if existing_class.contains(&"edge".to_string()) {
                    let _ = element.set_attribute("class", format!("edge {}", existing_class.join(" ")).as_str());
                }
                Some(())
            });
        });
    }

    match (*start_coordinates).clone().and_then(|start| (*end_coordinates).clone().and_then(|end| (*view_box).clone().map(|view_box| (start, end, view_box)))) {
        Ok((start_coordinates, end_coordinates, view_box)) => {
            html! {
                <svg
                    ref={edge_ref.clone()}
                    width={format!("{}px", view_box.2)}
                    height={format!("{}px", view_box.3)}
                    style={format!("position: absolute; left: {}px; top: {}px; overflow: visible;", view_box.0, view_box.1)}
                    xmlns="http://www.w3.org/2000/svg"
                >
                    {
                        edge_view.emit(EdgeViewProps {
                            edge: edge.clone(),
                            edge_coordinates: EdgeCoordinates {
                                start_coordinates: (start_coordinates.0 - view_box.0, start_coordinates.1 - view_box.1),
                                end_coordinates: (end_coordinates.0 - view_box.0, end_coordinates.1 - view_box.1)
                            },
                        })
                    }
                </svg>
            }
        },
        Err(e) => {
            warn!(e);
            html! {}
        }
    }

}