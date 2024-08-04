use yew::{classes, function_component, html, use_context, Html};

use crate::{
    components::{frequencies::Frequencies, instructions::Instructions},
    Context,
};

#[function_component]
pub fn Cruise() -> Html {
    let _ctx = use_context::<Context>().expect("no ctx found");

    html! {
        <div class={classes!("pr-0", "mr-0", "container")} style={"padding-right: 0;"}>
            <div class={classes!("row")}>
                    <div class={classes!("col-5", "p-0")}>
                            <Frequencies total_lines={8} name_width={"60%".to_string()} freq_width={"40%".to_string()} show_unicom={true} show_center_stations={true}/>
                    </div>
                    <div class={classes!("col-7", "pl-1", "pr-0")}>
                            <Instructions lines={8} />
                    </div>
            </div>
        </div>
    }
}

//<Infobox label={"CRZ FL:"} />
