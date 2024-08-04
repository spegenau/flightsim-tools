use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};

use crate::{
    components::{
        frequencies::{Frequencies, Frequency},
        infobox::Alignment,
    },
    Context,
};

use super::infobox;

#[function_component]
pub fn Taxi() -> Html {
    let ctx = use_context::<Context>().expect("no ctx found");

    let runway: String = ctx.simbrief.api_params.origrwy.clone();

    let mut ground = Frequency {
        name: "Ground".to_string(),
        frequency: String::default(),
    };

    let mut tower = Frequency {
        name: "Tower".to_string(),
        frequency: String::default(),
    };

    let mut departure = Frequency {
        name: "Departure".to_string(),
        frequency: String::default(),
    };

    let origin = ctx.simbrief.origin.icao_code.as_str();

    let controllers = ctx.vatsim.get_controllers_by_callsign(origin);

    for (callsign, ctr) in controllers {
        if callsign.ends_with("GND") {
            ground.frequency.push_str(ctr.frequency.as_str());
        } else if callsign.ends_with("TWR") {
            tower.frequency.push_str(ctr.frequency.as_str());
        } else if callsign.ends_with("APP") {
            departure.frequency.push_str(ctr.frequency.as_str());
        }
    }

    let frequencies: Vec<Frequency> = vec![ground, tower, departure];

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
