use yew::{function_component, Html, html};

use crate::node::{drag_handle::DragHandle, handle::{Handle, HandleType}};

use super::node_view_wrapper::NodeViewProps;

#[derive(PartialEq, Clone)]
pub struct NodeView2Data {
    pub source_count: usize,
    pub target_count: usize
}

#[function_component(NodeView2)]
pub fn node_view2(props: &NodeViewProps<NodeView2Data>) -> Html {

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