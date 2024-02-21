
use yewflow::viewport::use_viewport::{use_viewport, UseViewport};
use yewflow::yew_flow_provider::YewFlowProvider;
use yewflow::edge::edge_model::EdgeModel;
use yewflow::node::node_model::NodeModel;
use yew::prelude::*;
use yewflow::panel::Panel;

#[function_component(MyFlow)]
pub fn my_flow() -> Html {

    let UseViewport { viewport: _, set_viewport: _, center } = use_viewport::<(), ()>();
    
    let nodes = vec![
        NodeModel {
            id: String::from("0"),
            position: (0.0, 0.0),
            data: (),
        },
        NodeModel {
            id: String::from("1"),
            position: (200.0, 0.0),
            data: (),
        },
        NodeModel {
            id: String::from("2"),
            position: (0.0, 200.0),
            data: (),
        },
        NodeModel {
            id: String::from("3"),
            position: (200.0, 200.0),
            data: (),
        },
    ];

    let edges = vec![
        EdgeModel {
            id: String::from("0_1_edge"),
            start_id: String::from("0"),
            end_id: String::from("1"),
            source_handle_id: String::from("0_source"),
            target_handle_id: String::from("1_target"),
            data: (),
        },
        EdgeModel {
            id: String::from("0_2_edge"),
            start_id: String::from("0"),
            end_id: String::from("2"),
            source_handle_id: String::from("0_source"),
            target_handle_id: String::from("2_target"),
            data: (),
        },
        EdgeModel {
            id: String::from("2_3_edge"),
            start_id: String::from("2"),
            end_id: String::from("3"),
            source_handle_id: String::from("2_source"),
            target_handle_id: String::from("3_target"),
            data: (),
        }
    ];

    let on_center = {
        let center = center.clone();
        Callback::<MouseEvent, ()>::from(move |_| {
            center.emit(());
        })
    };
    
    html! {
        <>
            <button
                style={"position: absolute; left: 50%; top: 0; transform: translateX(-50%);"}
                onclick={on_center}
            >
                {"Click me to center the flow!"}
            </button>
            <Panel
                nodes={nodes}
                edges={edges}
                width="100vw"
                height="100vh"
            />
        </>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <YewFlowProvider>
            <MyFlow />
        </YewFlowProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
