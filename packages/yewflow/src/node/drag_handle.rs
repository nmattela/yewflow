use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DragHandleProps {
    /// Optional child elements for the handle
    #[prop_or_default]
    pub children: Html,
    /// Optional CSS class
    #[prop_or_default]
    pub class: String,
    /// Optional CSS style
    #[prop_or_default]
    pub style: String
}

/**
 * A drag handle is a handle which lets you drag the node around
 */
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