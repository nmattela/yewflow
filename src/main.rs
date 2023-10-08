mod app;
mod panel;
mod utils;
mod node;
mod edge;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
