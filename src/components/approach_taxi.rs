use crate::{
    components::{
        flexibox::{Flexibox, FlexiboxEntry},
        frequencies::{Frequencies, Frequency},
        infobox::Alignment,
    },
    vatsim::vatsim_data_manager::{self, ControllerLine, ControllerType, VatsimDataManager},
    Context,
};
use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};

use super::infobox;

#[function_component]
pub fn ApproachTaxi() -> Html {
    let ctx = use_context::<Context>().expect("no ctx found");

    let runway: String = ctx.simbrief.api_params.destrwy.clone();

    let vatsim_data_manager = VatsimDataManager {
        vatsim: ctx.vatsim,
        transceivers: ctx.transceivers.clone(),
    };

    let destination = ctx.simbrief.destination.icao_code.as_str();
    let stations_for_airport = vatsim_data_manager.get_stations_for_airport(destination.clone());

    let approach = Frequency {
        name: "Approach".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Approach)
            .unwrap_or(&ControllerLine::default())
            .frequencies
            .join(", "),
    };

    let tower = Frequency {
        name: "Tower".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Tower)
            .unwrap_or(&ControllerLine::default())
            .frequencies
            .join(", "),
    };

    let ground = Frequency {
        name: "Ground".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Ground)
            .unwrap_or(&ControllerLine::default())
            .frequencies
            .join(", "),
    };

    let apron = Frequency {
        name: "Apron".to_string(),
        frequency: stations_for_airport
            .get(&ControllerType::Delivery)
            .unwrap_or(&ControllerLine::default())
            .frequencies
            .join(", "),
    };

    let frequencies: Vec<Frequency> = vec![approach, tower, ground, apron];

    let size_left = "col-3".to_string();
    let size_right = "col-9".to_string();
    let min_height = "1.5cm".to_string();

    let entries = vec![
        FlexiboxEntry::label_only("TL"),
        FlexiboxEntry::label_only("Wind"),
        FlexiboxEntry::label_only("Temp"),
        FlexiboxEntry::label_only("QNH"),
    ];

    html! {
        <div class={classes!("container")}>
            <div class={classes!("row")}>
                <div class={classes!("col-4", "p-0")}>
                    <Frequencies show_unicom={false} frequencies={frequencies} name_width={"35%".to_string()} freq_width={"65%".to_string()}/>
                </div>
                <div class={classes!("col-2", "ml-1")}>
                    <Infobox label={"STAR"} align={Alignment::Vertical} proposition={ctx.simbrief.general.get_star()} min_height={".93cm".to_string()}/>
                    <Infobox class={classes!("mt-2")} label={"Runway"} proposition={runway}  align={Alignment::Vertical} min_height={".93cm".to_string()}/>
                    <Infobox class={classes!("mt-2")} label={"Gate"}  align={Alignment::Vertical} min_height={".93cm".to_string()}/>
                </div>


                <div class={classes!("col-6", "ml-1")}>
                    <Infobox label={"Awaited"} size_left={size_left.clone()} size_right={size_right.clone()} min_height={min_height.clone()} dense={true}  />
                    <Infobox class={classes!("mt-2")} label={"Confirmed"} size_left={size_left.clone()} size_right={size_right.clone()} min_height={min_height.clone()} dense={true} />
                    <Flexibox {entries} />
                </div>
            </div>
        </div>
    }
}
