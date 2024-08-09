use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};

use crate::simbrief::{airport::Airport, general::General, simbrief_response::SimbriefResponse};

use super::infobox;

#[function_component]
pub fn GeneralInformation() -> Html {
    let simbrief = use_context::<SimbriefResponse>().expect("no ctx found");

    let general: General = simbrief.general.clone();
    let origin: Airport = simbrief.origin.clone();
    let destination: Airport = simbrief.destination.clone();

    let size_left = "col-4".to_string();
    let size_right = "col-8".to_string();

    html! {
        <div class={classes!("container")}>
            <div class={classes!("row")}>
                <div class={classes!("col-4")}>
                    <Infobox label={"Callsign:"} size_left={size_left.clone()} size_right={size_right.clone()} >
                        {general.get_call_sign()}
                    </Infobox>
                </div>
                <div class={classes!("col-1")} />
                <div class={classes!("col-7")}>
                    <Infobox label={"Departure:"} size_left={size_left.clone()} size_right={size_right.clone()} >
                        {origin.format_name()}
                    </Infobox>
                    <Infobox class={classes!("mt-1")} label={"Arrival:"} size_left={size_left.clone()} size_right={size_right.clone()}>
                        {destination.format_name()}
                    </Infobox>
                </div>
            </div>
        </div>
    }
}
