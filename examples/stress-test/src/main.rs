use yew::prelude::*;
use yewflow::{edge::{edge_model::EdgeModel, edge_view_wrapper::{EdgeCoordinates, EdgeViewProps}}, node::{handle::{Handle, HandleType}, node_model::NodeModel, node_view_wrapper::NodeViewProps}, panel::Panel};

#[function_component(NodeView)]
pub fn node_view(props: &NodeViewProps<()>) -> Html {

    let NodeViewProps { node } = props;

    html! {
        <div style={"position: relative; border: 1px solid black; background-color: black; width: 70px; height: 35px;"}>
            <Handle
                id={format!("{}_target", node.id.clone())}
                handle_type={HandleType::Target}
                style={"width: 5px; height: 5px; background-color: red; position: absolute; left: 0; top: 50%; transform: translate(0, -50%);"}
            />
            <Handle
                id={format!("{}_source", node.id.clone())}
                handle_type={HandleType::Source}
                style={"width: 5px; height: 5px; background-color: blue; position: absolute; right: 0; top: 50%; transform: translate(0, -50%);"}
            />
        </div>
    }

}

#[function_component(EdgeView)]
pub fn edge_view(props: &EdgeViewProps<()>) -> Html {

    let EdgeViewProps { edge: _, edge_coordinates: EdgeCoordinates { start_coordinates, end_coordinates } } = props;

    html! {
        <line
            x1={(start_coordinates.0).to_string()}
            y1={(start_coordinates.1).to_string()}
            x2={(end_coordinates.0).to_string()}
            y2={(end_coordinates.1).to_string()}
            stroke={"black"}
            stroke-width={"5px"}
        />
    }
}

#[function_component(App)]
pub fn app() -> Html {

    let nodes = use_state(|| vec![0; 200].iter().enumerate().map(|(i, _)| NodeModel{
        id: i.to_string(),
        position: ((i as f64 % 100.0) * 100.0, ((i / 100) as f64).floor() * 100.0),
        data: (),
    }).collect::<Vec<NodeModel<()>>>());

    let edges = use_state(|| (*(nodes.clone())).iter().enumerate().flat_map(|(i, _node)| {
        if i != nodes.clone().len() -1 {
            vec![
                EdgeModel{
                    id: format!("{}-{}", i, i+1),
                    start_id: i.to_string(),
                    end_id: (i+1).to_string(),
                    source_handle_id: format!("{}_source", i),
                    target_handle_id: format!("{}_target", i+1),
                    data: (),
                }
            ]
        } else {
            vec![]
        }
    }).collect::<Vec<EdgeModel<()>>>());

    html! {
        <Panel<(), ()>
            nodes={(*nodes).clone()}
            edges={(*edges).clone()}
            width="100vw"
            height="100vh"
            node_view={Callback::from(|props| html! { <NodeView ..props /> } )}
            edge_view={Callback::from(|props| html! { <EdgeView ..props /> } )}
            style={"background-color: gray;"}
            debug={true}
        />
    }

}

fn main() {
    yew::Renderer::<App>::new().render();
}
