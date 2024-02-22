use yew::prelude::*;

use crate::viewport::viewport_struct::Viewport;

#[derive(Properties, PartialEq)]
pub struct YewFlowProviderProps {
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct YewFlowContext {
    pub viewport: UseStateHandle<Viewport>,
    pub panel_ref: UseStateHandle<Option<NodeRef>>,
}

/**
 * The YewFlowProvider is a provider for some of the context that is necessary for YewFlow to work
 * This provider enables the use_viewport() hook, which lets you programatically change the viewport position
 */
#[function_component(YewFlowProvider)]
pub fn yew_flow_provider(props: &YewFlowProviderProps) -> Html {

    let YewFlowProviderProps { children } = props;

    let viewport: UseStateHandle<Viewport> = use_state(|| Viewport::new(0.0, 0.0, 1.0));
    let panel_ref: UseStateHandle<Option<NodeRef>> = use_state(|| None);

    html! {
        <ContextProvider<YewFlowContext> context={YewFlowContext{ viewport, panel_ref }}>
            {children.clone()}
        </ContextProvider<YewFlowContext>>
    }

}