use yew::{function_component, html, Html};

use crate::components::controllers::Controllers;

#[function_component]
pub fn ControllersOnline() -> Html {
    html! {
        <div>
            <h3>{"Controllers Online"}</h3>
            <Controllers />
        </div>
    }
}
