mod app;
mod button;
mod parser;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
