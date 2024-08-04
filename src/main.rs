use components::header::Header;
use vatsim::transceiver;
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
use components::vatsim_loader::VatsimLoader;
use components::vatsim_transceiver_loader::VatsimTransceiverLoader;
use route::switch_route;
use route::Route;

#[derive(Clone, Debug, PartialEq, Default)]
struct Context {
    simbrief: SimbriefResponse,
    vatsim: VatsimResponse,
    transceivers: Vec<Transceiver>,
}

#[function_component]
fn App() -> Html {
    let ctx = use_state(Context::default);

    let on_simbrief_update: Callback<SimbriefResponse> = {
        let ctx = ctx.clone();
        Callback::from(move |simbrief| {
            ctx.set(Context {
                simbrief,
                vatsim: ctx.vatsim.clone(),
                transceivers: ctx.transceivers.clone(),
            });
        })
    };

    let on_vatsim_update: Callback<VatsimResponse> = {
        let ctx = ctx.clone();
        Callback::from(move |vatsim| {
            ctx.set(Context {
                vatsim,
                simbrief: ctx.simbrief.clone(),
                transceivers: ctx.transceivers.clone(),
            });
        })
    };

    let on_vatsim_transceiver_update: Callback<Vec<Transceiver>> = {
        let ctx = ctx.clone();
        Callback::from(move |transceivers| {
            ctx.set(Context {
                vatsim: ctx.vatsim.clone(),
                simbrief: ctx.simbrief.clone(),
                transceivers,
            });
        })
    };

    html! {
        <div>
            <VatsimLoader on_vatsim_update={on_vatsim_update}/>
            <VatsimTransceiverLoader {on_vatsim_transceiver_update}/>
            <ContextProvider<Context> context={(*ctx).clone()}>
                <Header {on_simbrief_update}/>
                <div class={classes!("container")}>
                    <BrowserRouter>
                        <Switch<Route> render={switch_route} />
                    </BrowserRouter>
                </div>
            </ContextProvider<Context>>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
