use gloo_console::log;
use yew::{classes, function_component, html, use_context, Html};

use crate::{
    components::{frequencies::Frequencies, instructions::Instructions},
    simbrief::simbrief_response::SimbriefResponse,
    vatsim::transceiver::Transceiver,
};

use super::frequencies::Frequency;
use itertools::Itertools;

#[function_component]
pub fn Cruise() -> Html {
    let simbrief = use_context::<SimbriefResponse>().expect("no ctx found");
    let transceivers = use_context::<Vec<Transceiver>>().expect("no ctx found");

    let transceivers: Vec<Transceiver> = transceivers.clone();

    log!(format!(
        "Found {} transceivers in VATSIM",
        transceivers.len()
    ));

    let mut callsigns: Vec<String> = Vec::new();

    let relevant_transceivers: Vec<Transceiver> = transceivers
        .iter()
        .filter(|t| t.callsign.ends_with("_CTR"))
        .cloned()
        .collect();

    for point in simbrief.navlog.fix {
        for transceiver in &relevant_transceivers {
            if let Some((callsign, frequency)) =
                transceiver.get_callsign_and_frequency(&point.as_lat_long())
            {
                log!(format!("Found transceiver {}", callsign.clone()));
                callsigns.push(format!("{}#####{}", callsign.clone(), frequency.clone()));
            }
        }
    }

    callsigns.dedup();

    let frequencies: Vec<Frequency> = callsigns
        .iter()
        .unique()
        .map(|c| {
            let mut split = c.split("#####");
            let callsign = split.next().unwrap().to_string();
            let frequency = split.next().unwrap().to_string();

            Frequency {
                id: 0,
                callsign,
                frequency,
            }
        })
        .collect();

    log!(format!("Found frequencies {:#?}", frequencies.len()));

    html! {
        <div class={classes!("pr-0", "mr-0", "container")} style={"padding-right: 0;"}>
            <div class={classes!("row")}>
                    <div class={classes!("col-5", "p-0")}>
                            <Frequencies total_lines={8} {frequencies} name_width={"60%".to_string()} freq_width={"40%".to_string()} show_unicom={true}/>
                    </div>
                    <div class={classes!("col-7", "pl-1", "pr-0")}>
                            <Instructions lines={8} />
                    </div>
            </div>
        </div>
    }
}

//<Infobox label={"CRZ FL:"} />
