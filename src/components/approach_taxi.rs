use crate::{
    components::{
        flexibox::{Flexibox, FlexiboxEntry},
        frequencies::{Frequencies, Frequency},
        infobox::Alignment,
    },
    simbrief::simbrief_response::SimbriefResponse,
    vatsim::{
        transceiver::Transceiver,
        vatsim_data_manager::{ControllerLine, ControllerType, VatsimDataManager},
        vatsim_response::VatsimResponse,
    },
};
use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};

use super::infobox;

#[function_component]
pub fn ApproachTaxi() -> Html {
    let simbrief = use_context::<SimbriefResponse>().expect("no ctx found");
    let vatsim = use_context::<VatsimResponse>().expect("no ctx found");
    let transceivers = use_context::<Vec<Transceiver>>().expect("no ctx found");

    let runway: String = simbrief.api_params.destrwy.clone();
    let destination = simbrief.destination.icao_code.as_str();

    let atis = vatsim
        .get_atis_for_airport(
            destination,
            crate::vatsim::vatsim_response::AtisType::Arrival,
        )
        .unwrap_or_default();

    let vatsim_data_manager = VatsimDataManager { transceivers };

    let stations_for_airport = vatsim_data_manager.get_stations_for_airport(destination);

    let approach = Frequency {
        id: 0,
        callsign: "Approach".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Approach)
            .unwrap_or(&ControllerLine::default())
            .format_frequencies(),
    };

    let tower = Frequency {
        id: 1,
        callsign: "Tower".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Tower)
            .unwrap_or(&ControllerLine::default())
            .format_frequencies(),
    };

    let ground = Frequency {
        id: 2,
        callsign: "Ground".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Ground)
            .unwrap_or(&ControllerLine::default())
            .format_frequencies(),
    };

    let apron = Frequency {
        id: 3,
        callsign: "Apron".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Delivery)
            .unwrap_or(&ControllerLine::default())
            .format_frequencies(),
    };

    let frequencies: Vec<Frequency> = vec![approach, tower, ground, apron];

    let size_left = "col-3".to_string();
    let size_right = "col-9".to_string();
    let min_height = "1.5cm".to_string();

    let entries = vec![
        FlexiboxEntry::label_and_proposition("Atis Info", atis.information_letter.as_str()),
        FlexiboxEntry::label_and_proposition("TL", atis.transition_level.as_str()),
        FlexiboxEntry::label_and_proposition("Wind", atis.format_wind().as_str()),
        FlexiboxEntry::label_and_proposition("Temp", atis.temperature.as_str()),
        FlexiboxEntry::label_and_proposition("QNH", atis.altimeter_settings.as_str()),
    ];

    html! {
        <div class={classes!("container")}>
            <div class={classes!("row")}>
                <div class={classes!("col-4", "p-0")}>
                    <Frequencies show_unicom={false} frequencies={frequencies} name_width={"35%".to_string()} freq_width={"65%".to_string()}/>
                </div>
                <div class={classes!("col-2", "ml-1")}>
                    <Infobox label={"STAR"} align={Alignment::Vertical} proposition={simbrief.general.get_star()} min_height={".93cm".to_string()}/>
                    <Infobox class={classes!("mt-2")} label={"Runway"} proposition={runway}  align={Alignment::Vertical} min_height={".93cm".to_string()}/>
                    <Infobox class={classes!("mt-2")} label={"Gate"}  align={Alignment::Vertical} min_height={".93cm".to_string()}/>
                </div>


                <div class={classes!("col-6", "ml-1", "paddingRightZero")}>
                    <div class={classes!("row")}>
                        <Infobox class={classes!("paddingRightZero")} label={"Awaited"} size_left={size_left.clone()} size_right={size_right.clone()} min_height={min_height.clone()} dense={true}  />
                    </div>
                    <div class={classes!("row")}>
                        <Infobox class={classes!("mt-2", "paddingRightZero")} label={"Confirmed"} size_left={size_left.clone()} size_right={size_right.clone()} min_height={min_height.clone()} dense={true} />
                    </div>
                    <div class={classes!("row")}>
                        <Flexibox {entries} class={classes!("paddingLeftZero")}/>
                    </div>
                </div>
            </div>
        </div>
    }
}
