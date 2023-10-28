
use yew::prelude::*;

use crate::{edge::edge_view_wrapper::EdgeCoordinates};

use super::{edge_view_wrapper::EdgeViewProps};

#[function_component(DefaultPreviewEdgeView)]
pub fn default_preview_edge_view(props: &EdgeViewProps<()>) -> Html {

    let EdgeViewProps { edge: _, edge_coordinates: EdgeCoordinates { start_coordinates, end_coordinates } } = props;

    html! {
        <line
            x1={(start_coordinates.0).to_string()}
            y1={(start_coordinates.1).to_string()}
            x2={(end_coordinates.0).to_string()}
            y2={(end_coordinates.1).to_string()}
            stroke="black"
        />
    }
}