use yew::prelude::*;

pub const SOURCE_HANDLE_CLASS: &str = "__yewflow_source_handle__";
pub const TARGET_HANDLE_CLASS: &str = "__yewflow_target_handle__";

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
    
    #[prop_or(true)]
    pub is_connectable: bool,
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
        is_connectable,
    } = props;

    let handle_ref = use_node_ref();

    html! {
        <div
            ref={handle_ref.clone()}
            id={id.clone()}
            class={format!(
                "{} {} handle",
                class.clone().unwrap_or(implicit_clone::unsync::IString::Static("")),
                match handle_type { HandleType::Source => SOURCE_HANDLE_CLASS, HandleType::Target => TARGET_HANDLE_CLASS }
            )}
            style={style}
            is_connectable={is_connectable.clone().to_string()}
        >
        </div>
    }
}