use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod routes;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<routes::AppRoute> render={routes::switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
