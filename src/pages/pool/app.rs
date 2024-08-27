use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h1>{ "App Component" }</h1>
            { /* Your global state or context setup */ }
        </div>
    }
}
