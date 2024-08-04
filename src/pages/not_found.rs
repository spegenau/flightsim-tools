use yew::{function_component, html, Html};

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div>
            <h3>{"Not found"}</h3>
        </div>
    }
}
