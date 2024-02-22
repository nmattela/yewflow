use yew::prelude::*;

use crate::edge::edge_view_wrapper::EdgeCoordinates;

use super::builtin_edge_props::BuiltinEdgeProps;



#[function_component(BezierEdge)]
pub fn bezier_edge(props: &BuiltinEdgeProps) -> Html {

    let EdgeCoordinates { start_coordinates, end_coordinates } = props.edge_coordinates;

    html! {
        <path
            d={format!(
                "M {} {} C {} {}, {} {}, {} {}",
                start_coordinates.0,
                start_coordinates.1,
                end_coordinates.0 / 2.0,
                start_coordinates.1,
                start_coordinates.0 + (end_coordinates.0 / 2.0),
                end_coordinates.1,
                end_coordinates.0,
                end_coordinates.1,
            )}
            fill={"transparent"}
            stroke={props.stroke.clone()}
            style={props.style.clone()}
            class={props.class.clone()}
            stroke-width={props.stroke_width.clone()}
            marker-end={props.marker_end.clone()}
            onclick={props.onclick.clone()}
        />
    }

}