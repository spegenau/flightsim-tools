use yew::{function_component, html, use_context, Html};

use crate::components::controllers::Controllers;
use crate::Context;

#[function_component]
pub fn ControllersOnline() -> Html {
    let _ctx = use_context::<Context>().expect("no ctx found");
    html! {
        <div>
            <h3>{"Controllers Online"}</h3>
            <Controllers />
        </div>
    }
}
