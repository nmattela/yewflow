
use yew::prelude::*;





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