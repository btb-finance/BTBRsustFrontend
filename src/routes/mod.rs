use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::calculator::Calculator;
use crate::pages::pool::Pool;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/calculator")]
    Calculator,
    #[at("/pool")]
    Pool,
    #[at("/")]
    Home,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Calculator => html! { <Calculator /> },
        AppRoute::Pool => html! { <Pool /> },
        AppRoute::Home => html! { <div>{ "Home" }</div> },
    }
}
