use web_sys::HtmlElement;
use yew::{Callback, Properties, function_component, Html, use_node_ref, use_effect, NodeRef};
use yew_hooks::UseMapHandle;


use crate::utils::{Position, AttributeExtractHelper};

use super::node_model::NodeModel;

#[derive(Properties, PartialEq)]
pub struct NodeViewWrapperProps<T: PartialEq + Clone> {
    pub node: NodeModel<T>,
    pub handle_registry: UseMapHandle<String, Position>,

    pub node_view: Callback<NodeViewProps<T>, Html>,
}

#[derive(Properties, PartialEq)]
pub struct NodeViewProps<T: PartialEq + Clone> {
    pub node: NodeModel<T>,

    pub node_ref: NodeRef,
}

/**
 * A wrapping component around a provided node component. It mostly handles the placement of the node.
 */
#[function_component(NodeViewWrapper)]
pub fn node_view_wrapper<T: PartialEq + Clone>(props: &NodeViewWrapperProps<T>) -> Html {
    
    let NodeViewWrapperProps { node, handle_registry: _, node_view } = props;
    
    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        let node = node.clone();
        use_effect(move || {
            node_ref.cast::<HtmlElement>().map(|element| {

                element.set_id(&node.id);

                let existing_class = element.get_class_names();
                if !existing_class.contains(&"node".to_string()) {
                    let _ = element.set_attribute("class", format!("node {}", existing_class.join(" ")).as_str());
                }
                let binding = element.get_attribute("style").unwrap_or_default();
                let existing_style = binding.split(';').collect::<Vec<&str>>();
                let contains_position = existing_style.iter().enumerate().find(|(_, s)| s.contains("position:")).map(|(i, _)| i);
                let contains_left = existing_style.iter().enumerate().find(|(_, s)| s.contains("left:")).map(|(i, _)| i);
                let contains_top = existing_style.iter().enumerate().find(|(_, s)| s.contains("top:")).map(|(i, _)| i);

                let with_position = match contains_position {
                    Some(contains_position) => existing_style.iter().enumerate().map(|(i, s)| if contains_position == i { "position: absolute".to_string() } else { s.to_string() }).collect::<Vec<String>>(),
                    None => existing_style.iter().chain(vec!["position: absolute"].iter()).map(|s| s.to_string()).collect::<Vec<String>>()
                };

                let with_left = match contains_left {
                    Some(contains_left) => with_position.iter().enumerate().map(|(i, s)| if contains_left == i { format!("left: {}px", node.position.0) } else { s.to_string() }).collect::<Vec<String>>(),
                    None => with_position.iter().chain(vec![format!("left: {}px", node.position.0)].iter()).map(|s| s.to_string()).collect::<Vec<String>>()
                };

                let with_top = match contains_top {
                    Some(contains_top) => with_left.iter().enumerate().map(|(i, s)| if contains_top == i { format!("top: {}px", node.position.1) } else { s.to_string() }).collect::<Vec<String>>(),
                    None => with_left.iter().chain(vec![format!("top: {}px", node.position.1)].iter()).cloned().collect::<Vec<String>>()
                };

                let _ = element.set_attribute("style", &with_top.join(";"));
                Some(())
            });
        });
    }

    node_view.emit(NodeViewProps {
        node: node.clone(),
        node_ref: node_ref.clone(),
    })

}