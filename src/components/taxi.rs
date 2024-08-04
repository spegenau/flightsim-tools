use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};

use crate::{
    components::{
        frequencies::{Frequencies, Frequency},
        infobox::Alignment,
    },
    vatsim::vatsim_data_manager::{self, ControllerLine, ControllerType, VatsimDataManager},
    Context,
};

use super::infobox;

#[function_component]
pub fn Taxi() -> Html {
    let ctx = use_context::<Context>().expect("no ctx found");

    let runway: String = ctx.simbrief.api_params.origrwy.clone();
    let origin = ctx.simbrief.origin.icao_code.as_str();

    let vatsim_data_manager = VatsimDataManager {
        vatsim: ctx.vatsim,
        transceivers: ctx.transceivers.clone(),
    };
    let stations_for_airport = vatsim_data_manager.get_stations_for_airport(origin);

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

    let frequencies: Vec<Frequency> = vec![tower, ground, approach];

    let size_left = "col-3".to_string();
    let size_right = "col-9".to_string();
    let min_height = "1.5cm".to_string();

    html! {
        <div class={classes!("container")}>
            <div class={classes!("row")}>
                <div class={classes!("col-4", "p-0")}>
                    <Frequencies show_unicom={false} frequencies={frequencies} name_width={"35%".to_string()} freq_width={"65%".to_string()}/>
                </div>
                <div class={classes!("col-2", "ml-1")}>
                    <Infobox label={"Holding Point"} align={Alignment::Vertical} min_height={".93cm".to_string()}/>
                    <Infobox class={classes!("mt-2")} label={"Runway"} proposition={runway} align={Alignment::Vertical} min_height={".93cm".to_string()}/>
                </div>


                <div class={classes!("col-6", "ml-1")}>
                    <Infobox label={"Awaited"} size_left={size_left.clone()} size_right={size_right.clone()} min_height={min_height.clone()} dense={true}  />
                    <Infobox class={classes!("mt-2")} label={"Confirmed"} size_left={size_left.clone()} size_right={size_right.clone()} min_height={min_height.clone()} dense={true} />
                </div>
            </div>
        </div>
    }
}
