use yew::prelude::*;

#[function_component(Layout)]
pub fn layout() -> Html {
    html! {
        <div>
            <header>{ "Header" }</header>
            <main>
                { "This is the layout component." }
            </main>
            <footer>{ "Footer" }</footer>
        </div>
    }
}
