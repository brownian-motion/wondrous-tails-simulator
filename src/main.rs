mod body;
mod components;
mod stats;

fn main() {
    yew::Renderer::<body::App>::new().render();
}
