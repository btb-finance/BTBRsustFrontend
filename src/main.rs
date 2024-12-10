use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
mod pages;
mod routes;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Hello World!" }</h1>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
