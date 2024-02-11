use yew::prelude::*;

#[derive(Debug, PartialEq)]
pub enum HandleType {
    Source,
    Target,
}

#[derive(Properties, PartialEq)]
pub struct HandleProps {
    /// Uniquely identify the handle
    pub id: String,
    /// The handle must be either a source or a target
    pub handle_type: HandleType,
    
    /// Additional CSS style
    #[prop_or_default]
    pub style: Option<AttrValue>,
    /// Additional CSS class
    #[prop_or_default]
    pub class: Option<AttrValue>,
}

/**
 * A handle represents both endpoints of an edge on a node.
 */
#[function_component(Handle)]
pub fn handle(props: &HandleProps) -> Html {

    let HandleProps {
        handle_type,
        id,
        style,
        class,
    } = props;

    let handle_ref = use_node_ref();

    html! {
        <div
            ref={handle_ref.clone()}
            id={id.clone()}
            class={format!(
                "{} {} handle",
                class.clone().unwrap_or(implicit_clone::unsync::IString::Static("")),
                match handle_type { HandleType::Source => "source-handle", HandleType::Target => "target-handle" }
            )}
            style={style}
        >
        </div>
    }
}