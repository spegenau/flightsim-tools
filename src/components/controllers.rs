use yew::{classes, function_component, html, use_context, Html};

use crate::{vatsim::controller::Controller, Context};

#[function_component]
pub fn Controllers() -> Html {
    let ctx = use_context::<Context>().expect("no ctx found");
    let mut controllers: Vec<Controller> = ctx.vatsim.get_all_controllers();

    controllers.sort_by_key(|c| c.callsign.clone());

    html! {
    <div>
        <h3>{ controllers.len() }{ " online Controllers"}</h3>
        <table class={classes!("table", "table-striped", "table-bordered", "table-sm")}>
            <thead>
                <tr>
                    <th>{"CID"}</th>
                    <th>{"Name"}</th>
                    <th>{"Frequency"}</th>
                    <th>{"Callsign"}</th>
                    <th>{"Facility"}</th>
                    <th>{"Visual Range"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    controllers
                    .into_iter()
                    .map(|c| {
                        html! {
                            <tr key={c.cid.clone().to_string()}>
                                <td>{&c.cid.to_string()}</td>
                                <td>{&c.name}</td>
                                <td>{&c.frequency}</td>
                                <td>{&c.callsign}</td>
                                <td>{&c.facility.to_string()}</td>
                                <td>{&c.visual_range.to_string()}</td>
                            </tr>
                        }
                    })
                    .collect::<Html>()
                }
            </tbody>
        </table>
    </div>
    }
}
