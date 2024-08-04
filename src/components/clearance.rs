use infobox::Infobox;
use yew::{classes, function_component, html, use_context, Html};

use crate::{
    components::{
        frequencies::{Frequencies, Frequency},
        infobox::Alignment,
    },
    simbrief::general::General,
    Context,
};

use super::infobox;

#[function_component]
pub fn Clearance() -> Html {
    let ctx = use_context::<Context>().expect("no ctx found");

    let general: General = ctx.simbrief.general.clone();

    let mut delivery = Frequency {
        name: "Delivery".to_string(),
        frequency: String::default(),
    };

    let origin = ctx.simbrief.origin.icao_code.as_str();

    let controllers = ctx.vatsim.get_controllers_by_callsign(origin);

    for (callsign, ctr) in controllers {
        if callsign.ends_with("DEL") {
            delivery.frequency.push_str(ctr.frequency.as_str());
        }
    }

    let frequencies: Vec<Frequency> = vec![delivery];

    let padding = "ml-1";

    html! {
        <div class={classes!("container")}>
            <div class={classes!("row")}>
                <div class={classes!("col-4", "p-0")}>
                    <Frequencies {frequencies} name_width={"35%".to_string()} freq_width={"65%".to_string()}/>
                </div>
                <div class={classes!("col", padding)}>
                    <Infobox label={"Gate:"} dense={true}  min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                    <Infobox class={classes!("mt-1")} label={"SID:"} proposition={general.get_sid()} dense={true} min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                </div>
                <div class={classes!("col", padding)}>
                    <Infobox label={"ATIS:"}  dense={true}  min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                    <Infobox class={classes!("mt-1")} label={"Climb:"}  dense={true} min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                </div>
                <div class={classes!("col", padding, "p-0")}>
                    <Infobox label={"QNH:"}  dense={true}  min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                    <Infobox class={classes!("mt-1")} label={"Squawk:"}  dense={true} min_height={"1cm".to_string()} align={Alignment::Vertical}/>
                </div>
            </div>
        </div>
    }
}
