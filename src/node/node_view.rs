

use yew::prelude::*;
use yew_hooks::UseMapHandle;

use crate::{node::{drag_handle::DragHandle, handle::{Handle, HandleType}}, utils::Position};

use super::node_model::NodeModel;

#[derive(Properties, PartialEq)]
pub struct NodeViewProps {
    pub node: NodeModel,
    pub set_node: Callback<NodeModel>,

    pub handle_registry: UseMapHandle<String, Position>
}

#[function_component(NodeView)]
pub fn node_view(props: &NodeViewProps) -> Html {

    let NodeViewProps { node, set_node: _, handle_registry} = props;

    let node_id = &node.id;

    html!{
        <div
            id={node_id.clone()}
            class="node"
            style={format!("left: {}px; top: {}px", node.position.0, node.position.1)}
        >
            <Handle id={format!("to_{}", node_id.clone())} handle_type={HandleType::Target} handle_registry={handle_registry.clone()} node={node.clone()} />
            <DragHandle />
            <Handle id={format!("from_{}", node_id.clone())} handle_type={HandleType::Source} handle_registry={handle_registry.clone()} node={node.clone()} />
        </div>
    }


}