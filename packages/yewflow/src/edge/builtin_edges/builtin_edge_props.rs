use yew::prelude::*;

use crate::edge::edge_view_wrapper::EdgeCoordinates;


#[derive(Properties, PartialEq, Clone)]
pub struct BuiltinEdgeProps {
    pub edge_coordinates: EdgeCoordinates,
    #[prop_or(String::from("#000000"))]
    pub stroke: String,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub stroke_width: String,
    #[prop_or_default]
    pub marker_end: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent, ()>,
}