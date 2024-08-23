use yew::{function_component, html, Html};

use crate::components::atis_list::AtisList;

#[function_component]
pub fn AtisPage() -> Html {
    html! {
        <div>
            <h3>{"Atis"}</h3>
            <AtisList />
        </div>
    }
}
