use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};

use crate::{
    components::{
        flexibox::{Flexibox, FlexiboxEntry},
        frequencies::{Frequencies, Frequency},
        infobox::Alignment,
    },
    simbrief::{general::General, simbrief_response::SimbriefResponse},
    vatsim::vatsim_response::VatsimResponse,
};

use super::infobox;

#[function_component]
pub fn Clearance() -> Html {
    let simbrief = use_context::<SimbriefResponse>().expect("no ctx found");
    let vatsim = use_context::<VatsimResponse>().expect("no ctx found");

    let origin = simbrief.origin.icao_code.as_str();
    let atis = vatsim
        .get_atis_for_airport(origin, crate::vatsim::vatsim_response::AtisType::Departure)
        .unwrap_or_default();

    let general: General = simbrief.general.clone();

    let mut delivery = Frequency {
        id: 0,
        callsign: "Delivery".to_string(),
        frequency: String::default(),
    };

    let controllers = vatsim.get_controllers_by_callsign(origin);

    let mut frequencies: Vec<String> = Vec::new();
    for (callsign, ctr) in controllers {
        if callsign.ends_with("DEL") {
            frequencies.push(ctr.frequency.to_string());
        }
    }
    frequencies.sort();
    delivery.frequency = frequencies.join(", ");

    let frequencies: Vec<Frequency> = vec![delivery];

    let entries = vec![
        FlexiboxEntry::label_only("Gate"),
        FlexiboxEntry::label_and_proposition("ATIS Info", atis.information_letter.as_str()),
        FlexiboxEntry::label_and_proposition("Temp", atis.temperature.as_str()),
        FlexiboxEntry::label_and_proposition("QNH", atis.altimeter_settings.as_str()),
    ];

    let padding = "ml-1";

    html! {
        <div class={classes!("container")}>
            <div class={classes!("row")}>
                <div class={classes!("col-4", "p-0")}>
                    <Frequencies {frequencies} name_width={"35%".to_string()} freq_width={"65%".to_string()}/>
                </div>
                <div class={classes!("col-8", "paddingRightZero")}>
                    <div class={classes!("row", "paddingRightZero")}>
                        <Flexibox {entries} />
                    </div>
                    <div class={classes!("row", "paddingRightZero")}>
                        <div class={classes!("col", padding)}>
                            <Infobox class={classes!("mt-1")} label={"SID:"} proposition={general.get_sid()} dense={true} min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                        </div>
                        <div class={classes!("col", padding)}>
                            <Infobox class={classes!("mt-1")} label={"Climb:"}  dense={true} min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                        </div>
                        <div class={classes!("col", padding)}>
                            <Infobox class={classes!("mt-1")} label={"Squawk:"}  dense={true} min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
