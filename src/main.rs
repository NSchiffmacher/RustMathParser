mod app;
mod button;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
