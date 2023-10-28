use yew::prelude::*;


use crate::{node::{drag_handle::DragHandle, handle::{Handle, HandleType}}};

use super::{node_view_wrapper::NodeViewProps};

#[function_component(NodeView)]
pub fn node_view(props: &NodeViewProps<()>) -> Html {

    let NodeViewProps { node, node_ref} = props;

    let node_id = &node.id;

    html!{
        <div
            ref={node_ref}
            class={"node-view-1"}
            id={node_id.clone()}
            style={format!("left: {}px; top: {}px", node.position.0, node.position.1)}
        >
            <Handle
                id={format!("to_{}", node_id.clone())}
                handle_type={HandleType::Target}
                style={"background-color: blue;"}
            />
            <DragHandle class="drag-handle-node-view" />
            <Handle
                id={format!("from_{}", node_id.clone())}
                handle_type={HandleType::Source}
                style={"background-color: red;"}
            />
        </div>
    }


}