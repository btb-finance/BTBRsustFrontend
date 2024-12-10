use yew::prelude::*;

#[function_component(Calculator)]
pub fn calculator() -> Html {
    html! {
        <div class="calculator-page">
            <h1>{ "Calculator Page" }</h1>
            <div class="calculator">
                <div class="display">
                    <input type="text" readonly=true value="" />
                </div>
            </div>
        </div>
    }
}
