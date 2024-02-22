use yew::prelude::*;

use crate::node::{drag_handle::DragHandle, handle::{Handle, HandleType}};

use super::node_view_wrapper::NodeViewProps;

#[function_component(DefaultNodeView)]
pub fn default_node_view<T: PartialEq + Clone>(props: &NodeViewProps<T>) -> Html {

    let NodeViewProps { node } = props;

    html! {
        <div style={"width: 100px; height: 50px; border: 3px solid black;"}>
            <DragHandle style={"width: 100%; height: 100%"}>
                <Handle
                    id={format!("{}_target", node.id.clone())}
                    handle_type={HandleType::Target}
                    style={"width: 5px; height: 5px; border-radius: 1000px; background-color: red; position: absolute; left: 0; top: 50%; transform: translate(0, -50%);"}
                />
                <Handle
                    id={format!("{}_source", node.id.clone())}
                    handle_type={HandleType::Source}
                    style={"width: 5px; height: 5px; border-radius: 1000px; background-color: blue; position: absolute; right: 0; top: 50%; transform: translate(0, -50%);"}
                />
            </DragHandle>
        </div>
    }
}