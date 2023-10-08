mod app;
mod panel;
mod utils;
mod node;
mod edge;
mod hooks;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
