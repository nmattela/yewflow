use crate::{node::node_view_wrapper::NODE_CLASS, utils::AttributeExtractHelper};
use gloo_console::log;
use wasm_bindgen::JsCast;
use crate::viewport::viewport_struct::Viewport;
use crate::yew_flow_provider::YewFlowContext;
use web_sys::{HtmlCollection, HtmlElement};
use yew::prelude::*;

pub struct UseViewport {
    pub viewport: Viewport,
    pub set_viewport: Callback<Viewport, ()>,
    pub center: Callback<(), ()>,
}

/**
 * This hook gives you access to the viewport, including methods to manipulate the viewport
 */
#[hook]
pub fn use_viewport<NodeData: PartialEq + Clone + 'static, EdgeData: PartialEq + Clone + 'static>() -> UseViewport {

    let yew_flow_context = use_context::<YewFlowContext>().expect("You must wrap the component in which you are calling use_viewport() in a <YewFlowProvider/> component for this hook to function.");

    let viewport = use_memo(yew_flow_context.clone(), |yew_flow_context| {
        *(yew_flow_context.viewport)
    });

    let set_viewport = use_memo(yew_flow_context.clone(), |yew_flow_context| {
        let yew_flow_context = yew_flow_context.clone();
        Callback::from(move |new_viewport| yew_flow_context.viewport.set(new_viewport))
    });

    // Nodes are necessary to calculate the center of the viewport.
    // Nodes are extracted from the DOM, because we need to know their exact location and size
    let nodes = use_memo((*yew_flow_context.clone().panel_ref.clone()).clone(), |panel_ref| {
        fn get_node_elements(element: HtmlElement) -> Vec<HtmlElement> {
            element.children().dyn_into::<HtmlCollection>().map(|children| {
                let array = js_sys::Array::from(&children);
                array.into_iter().flat_map(|child| {
                    child.dyn_into::<HtmlElement>().map(|child| {
                        let class_names = child.get_class_names();
                        if class_names.contains(&NODE_CLASS.to_string()) {
                            vec![child]
                        } else {
                            get_node_elements(child)
                        }
                    }).unwrap_or(vec![])
                }).collect()
            }).unwrap_or(vec![])
        }
    
        panel_ref.clone().and_then(|panel_ref| panel_ref.get().and_then(|panel_ref| panel_ref.dyn_into::<HtmlElement>().ok().map(get_node_elements))).unwrap_or(vec![])
    });

    //A callback that centers the viewport perfectly in the middle
    //TODO: This will not yet zoom out the viewport to ensure all nodes fit on the screen
    let center = use_memo((yew_flow_context.clone().viewport.clone(), (*yew_flow_context.clone().panel_ref.clone()).clone(), nodes.clone()), |(viewport, panel_ref, nodes)| {
        let viewport = viewport.clone();
        let panel_ref = panel_ref.clone();
        

        let (lowest_x, highest_x, lowest_y, highest_y) = (*nodes.clone()).clone().into_iter().fold((f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY), |(lowest_x, highest_x, lowest_y, highest_y), node| {
            
            let dom_rect = node.get_bounding_client_rect();

            let node_lowest_x = dom_rect.x();
            let node_highest_x = dom_rect.x() + dom_rect.width();
            let node_lowest_y = dom_rect.y();
            let node_highest_y = dom_rect.y() + dom_rect.height();

            (
                if node_lowest_x < lowest_x { node_lowest_x } else { lowest_x },
                if node_highest_x > highest_x { node_highest_x } else { highest_x },
                if node_lowest_y < lowest_y { node_lowest_y } else { lowest_y },
                if node_highest_y > highest_y { node_highest_y } else { highest_y },
            )
        });
        
        log!((*nodes).clone().len());
        
        let width = highest_x - lowest_x;
        let height = highest_y - lowest_y;
        
        log!(lowest_x, highest_x, lowest_y, highest_y, width, height);

        Callback::<(), ()>::from(move |_| {
            if let Some(panel_ref) = panel_ref.clone() { viewport.set(viewport.center(panel_ref.clone(), (width, height))) }
        })
    });

    UseViewport {
        viewport: *viewport,
        set_viewport: (*set_viewport).clone(),
        center: (*center).clone(),
    }

}