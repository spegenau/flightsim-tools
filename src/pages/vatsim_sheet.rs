use yew::{function_component, html, Html};

use crate::components::cheatsheet::Cheatsheet;

#[function_component]
pub fn VatsimSheet() -> Html {
    html! {
        <div>
            <h3>{"Vatsim"}</h3>
            <Cheatsheet />
        </div>
    }
}
