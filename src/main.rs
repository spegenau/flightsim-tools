use components::header::Header;
use vatsim::transceiver::Transceiver;
use vatsim::vatsim_response::VatsimResponse;
use yew::prelude::*;
use yew_router::prelude::*;
mod simbrief;
use simbrief::simbrief_response::SimbriefResponse;
use yew::{function_component, html, use_state};
mod components;
mod pages;
mod route;
mod vatsim;
use components::debug_msg::DebugMessage;
use components::vatsim_loader::VatsimLoader;
use components::vatsim_transceiver_loader::VatsimTransceiverLoader;
use route::switch_route;
use route::Route;

#[derive(Clone, Debug, PartialEq, Default)]
struct MessagesContext {
    messages: Vec<String>,
}

#[function_component]
fn App() -> Html {
    let simbrief_context: UseStateHandle<_> = use_state(SimbriefResponse::default);
    let vatsim_context: UseStateHandle<_> = use_state(VatsimResponse::default);
    let transceiver_context: UseStateHandle<_> = use_state(Vec::default);
    let messages_context: UseStateHandle<_> = use_state(MessagesContext::default);

    let on_simbrief_update: Callback<SimbriefResponse> = {
        let ctx = simbrief_context.clone();
        Callback::from(move |simbrief| {
            ctx.set(simbrief);
        })
    };

    let on_vatsim_update: Callback<VatsimResponse> = {
        let ctx = vatsim_context.clone();
        Callback::from(move |vatsim| {
            ctx.set(vatsim);
        })
    };

    let on_vatsim_transceiver_update: Callback<Vec<Transceiver>> = {
        let ctx = transceiver_context.clone();
        Callback::from(move |transceivers| {
            ctx.set(transceivers);
        })
    };

    let on_error: Callback<Vec<String>> = {
        let ctx = messages_context.clone();
        Callback::from(move |new_messages: Vec<String>| {
            let mut messages: Vec<String> = ctx.messages.clone();
            messages.append(&mut new_messages.clone());
            ctx.set(MessagesContext { messages });
        })
    };

    html! {
        <div>
            <VatsimLoader on_vatsim_update={on_vatsim_update}  on_error={on_error.clone()}/>
            <VatsimTransceiverLoader {on_vatsim_transceiver_update} on_error={on_error.clone()}/>
            <ContextProvider<SimbriefResponse> context={(*simbrief_context).clone()} >
                <ContextProvider<VatsimResponse> context={(*vatsim_context).clone()} >
                    <ContextProvider<Vec<Transceiver>> context={(*transceiver_context).clone()} >
                        <ContextProvider<MessagesContext> context={(*messages_context).clone()} >
                            <Header {on_simbrief_update}  on_error={on_error.clone()} />
                            <div class={classes!("container")}>
                                <DebugMessage />
                                <BrowserRouter>
                                    <Switch<Route> render={switch_route} />
                                </BrowserRouter>
                            </div>
                        </ContextProvider<MessagesContext>>
                    </ContextProvider<Vec<Transceiver>>>
                </ContextProvider<VatsimResponse>>
            </ContextProvider<SimbriefResponse>>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
