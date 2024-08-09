use yew::{function_component, html, use_context, Html};

use crate::MessagesContext;

#[function_component]
pub fn DebugMessage() -> Html {
    let ctx = use_context::<MessagesContext>().expect("no ctx found");

    html! {
        <div>
            <h3>{"Debug"}</h3>
            <ul>
                {
                    ctx.messages.iter().map(|message| {
                        html!{
                            <li>{message}</li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
