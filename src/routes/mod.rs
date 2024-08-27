use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::calculator::Calculator;
use crate::pages::pool::Pool;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/calculator"]
    Calculator,
    #[to = "/pool"]
    Pool,
    #[to = "/"]
    Home,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Calculator => html! { <Calculator /> },
        AppRoute::Pool => html! { <Pool /> },
        AppRoute::Home => html! { <div>{ "Home" }</div> },
    }
}
