

use yew::{function_component, html, use_node_ref, Callback, Html, Properties};
use yew_hooks::UseMapHandle;

pub const NODE_CLASS: &str = "__yewflow_node__";

use crate::hooks::use_register_handles::Handle;

use super::node_model::NodeModel;

#[derive(Properties, PartialEq)]
pub struct NodeViewWrapperProps<T: PartialEq + Clone> {
    pub node: NodeModel<T>,
    pub handle_registry: UseMapHandle<String, Handle>,

    pub node_view: Callback<NodeViewProps<T>, Html>,
}

#[derive(Properties, PartialEq)]
pub struct NodeViewProps<T: PartialEq + Clone> {
    pub node: NodeModel<T>,
}

/**
 * A wrapping component around a provided node component. It mostly handles the placement of the node.
 */
#[function_component(NodeViewWrapper)]
pub fn node_view_wrapper<T: PartialEq + Clone>(props: &NodeViewWrapperProps<T>) -> Html {
    
    let NodeViewWrapperProps { node, handle_registry: _, node_view } = props;
    
    let node_ref = use_node_ref();

    html! {
        <div
            ref={node_ref.clone()}
            class={NODE_CLASS}
            id={node.id.clone()}
            style={format!("position: absolute; left: {}px; top: {}px", node.position.0, node.position.1)}
        >
            {node_view.emit(NodeViewProps {
                node: node.clone(),
            })}
        </div>
    }

}