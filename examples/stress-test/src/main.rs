use yew::prelude::*;
use yewflow::{edge::edge_model::EdgeModel, node::node_model::NodeModel, panel::Panel};

#[function_component(App)]
pub fn app() -> Html {

    let nodes = use_state(|| vec![0; 200].iter().enumerate().map(|(i, _)| NodeModel{
        id: i.to_string(),
        position: ((i as f64 % 100.0) * 150.0, ((i / 100) as f64).floor() * 100.0),
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
            style={"background-color: gray;"}
            debug={true}
        />
    }

}

fn main() {
    yew::Renderer::<App>::new().render();
}
