use yew::{html, Html};
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::pages::controllers_online::ControllersOnline;
use crate::pages::not_found::NotFound;
use crate::pages::vatsim_sheet::VatsimSheet;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/vatsim-sheet")]
    VatsimSheet,
    #[at("/controllers-online")]
    ControllersOnline,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_route(route: Route) -> Html {
    match route {
        Route::Home => html! { <VatsimSheet /> },
        Route::VatsimSheet => html! { <VatsimSheet /> },
        Route::About => html! { <About /> },
        Route::ControllersOnline => html! {
            <ControllersOnline />
        },
        Route::NotFound => html! { <NotFound/> },
    }
}
