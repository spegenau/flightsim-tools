use super::infobox;
use crate::simbrief::{airport::Airport, general::General, simbrief_response::SimbriefResponse};
use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};
use yew_bootstrap::component::*;

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
            <Row>
                <Column size=4>
                    <Row>
                        <Column>
                            <Infobox label={"Callsign:"} size_left={size_left.clone()} size_right={size_right.clone()} >
                                <span class="font-weight-bold" style="display: block;">{general.get_call_sign()}</span>
                                <span style="display: inline-block;">{format!("LVL: {}", general.get_flight_level())}</span>
                                <span style="display: inline-block; float: right;">{format!("CI: {}", general.costindex)}</span>
                            </Infobox>
                        </Column>
                    </Row>
                </Column>
                <Column size=1 />

                <Column size=7>
                    <Infobox label={"Departure:"} size_left={size_left.clone()} size_right={size_right.clone()} >
                        <span class="font-weight-bold">{origin.format_name()}</span>
                        <span style="display: inline-block; float: right;">{format!("{} / {}", simbrief.times.get_out_time(), simbrief.times.get_offblock_time())}</span>
                    </Infobox>
                    <Infobox class={classes!("mt-1")} label={"Arrival:"} size_left={size_left.clone()} size_right={size_right.clone()}>
                        <span class="font-weight-bold">{destination.format_name()}</span>
                        <span style="display: inline-block; float: right;">{format!("{} / {}", simbrief.times.get_onblock_time(), simbrief.times.get_in_time())}</span>
                    </Infobox>
                </Column>
            </Row>
        </div>
    }
}
