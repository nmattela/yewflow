use gloo_console::log;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlElement};

use super::node::Node;

#[derive(Properties, PartialEq)]
pub struct DragHandleProps {
}

#[function_component(DragHandle)]
pub fn drag_handle(props: &DragHandleProps) -> Html {

    let DragHandleProps {  } = props;

    html! {
        <div
            class="drag-handle"
        >
            
        </div>
    }

}