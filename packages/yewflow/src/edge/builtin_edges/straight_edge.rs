use yew::prelude::*;

use crate::edge::edge_view_wrapper::EdgeCoordinates;

use super::builtin_edge_props::BuiltinEdgeProps;

#[function_component(StraightEdge)]
pub fn straight_edge(props: &BuiltinEdgeProps) -> Html {

    let EdgeCoordinates { start_coordinates, end_coordinates } = props.edge_coordinates;

    html! {
        <line
            x1={(start_coordinates.0).to_string()}
            y1={(start_coordinates.1).to_string()}
            x2={(end_coordinates.0).to_string()}
            y2={(end_coordinates.1).to_string()}
            stroke={props.stroke.clone()}
            style={props.style.clone()}
            class={props.class.clone()}
            stroke-width={props.stroke_width.clone()}
            marker-end={props.marker_end.clone()}
            onclick={props.onclick.clone()}
        />
    }

}