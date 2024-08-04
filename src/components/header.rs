use yew::{classes, function_component, html, Callback, Html, Properties};

use crate::components::simbrief_loader::SimbriefLoader;
use crate::simbrief::simbrief_response::SimbriefResponse;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub on_simbrief_update: Callback<SimbriefResponse>,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    html! {
        <div class={classes!("container")}>
            <header class={classes!("d-flex", "flex-wrap", "justify-content-center", "py-3", "mb-4", "border-bottom")}>
                <a href="/" class={classes!("d-flex", "align-items-center", "mb-3", "mb-md-0", "me-md-auto", "link-body-emphasis", "text-decoration-none")}>
                    <span class={classes!("fs-4")}>{"Flight Simulation Tools"}</span>
                </a>
                <div class={classes!("me-md-auto")}>
                    <SimbriefLoader on_simbrief_update={props.on_simbrief_update.clone()}/>
                </div>

                <ul class={classes!("nav", "nav-pills")}>
                    <li class={classes!("nav-item")}><a href="/home" class={classes!("nav-link", "active")} aria-current="page">{"Home"}</a></li>
                    <li class={classes!("nav-item")}><a href="/vatsim-sheet" class={classes!("nav-link")}>{"Vatsim Sheet"}</a></li>
                    <li class={classes!("nav-item")}><a href="/controllers-online" class={classes!("nav-link")}>{"Controllers Online"}</a></li>
                    <li class={classes!("nav-item")}><a href="/about" class={classes!("nav-link")}>{"About"}</a></li>
                </ul>
            </header>
        </div>
    }
}
