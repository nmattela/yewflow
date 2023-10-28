
use yew::prelude::*;





#[derive(Properties, PartialEq)]
pub struct DragHandleProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub style: String
}

#[function_component(DragHandle)]
pub fn drag_handle(props: &DragHandleProps) -> Html {

    let DragHandleProps { children, class, style } = props;

    html! {
        <div
            class={format!("drag-handle {}", class)}
            style={style.to_string()}
        >
            {children.clone()}
        </div>
    }

}