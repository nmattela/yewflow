use yew::prelude::*;

use crate::edge::builtin_edges::straight_edge::StraightEdge;

use super::edge_view_wrapper::EdgeViewProps;

#[function_component(DefaultEdgeView)]
pub fn default_edge_view<T: PartialEq + Clone>(props: &EdgeViewProps<T>) -> Html {

    html! {
        <StraightEdge
            edge_coordinates={props.edge_coordinates.clone()}
        />
    }

}