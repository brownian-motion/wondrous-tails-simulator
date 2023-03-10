use wondrous_tails_simulator::body;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<body::App>::new().render();
}
