

use crate::{utils::Position};

use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::{UseMapHandle};



use super::node_model::NodeModel;

#[derive(Debug, PartialEq)]
pub enum HandleType {
    Source,
    Target,
}

#[derive(Properties, PartialEq)]
pub struct HandleProps {
    pub id: String,
    pub handle_type: HandleType,

    pub node: NodeModel,

    pub handle_registry: UseMapHandle<String, Position>
}

#[function_component(Handle)]
pub fn handle(props: &HandleProps) -> Html {

    let HandleProps { handle_type, id, handle_registry, node } = props;

    let node_ref = use_node_ref();

    {
        let handle_registry = handle_registry.clone();
        use_effect_with((id.clone(), node_ref.clone(), node.position), move |(id, node_ref, _node_position)| {
            node_ref.cast::<HtmlElement>().map(|node_ref| {
                let rect = node_ref.get_bounding_client_rect();
                let x = rect.x() as i32;
                let y = rect.y() as i32;
                let center_offset_x = (rect.width() / 3.0) as i32;
                let center_offset_y = (rect.height() / 3.0) as i32;
                let id = id.clone();
                handle_registry.insert(id, (x - center_offset_x, y - center_offset_y));
                Some(())
            });
        })
    };

    html! {
        <div
            ref={node_ref.clone()}
            id={id.clone()}
            class={format!("{} handle", match handle_type { HandleType::Source => "source-handle", HandleType::Target => "target-handle" })}
            style={format!("background-color: {};", match handle_type { HandleType::Source => "red", HandleType::Target => "blue" })}
        >
            
        </div>
    }
}