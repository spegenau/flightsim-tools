use yew::{classes, function_component, html, use_context, Html};

use crate::vatsim::{parsed_atis::ParsedAtis, vatsim_response::VatsimResponse};

#[function_component]
pub fn AtisList() -> Html {
    let vatsim = use_context::<VatsimResponse>().expect("no ctx found");

    let atis: Vec<ParsedAtis> = vatsim.get_parsed_atis_list();
    /*
    .clone()
    .iter()
    //.filter(|a| a.temperature.is_empty())
    .cloned()
    .collect();
    */

    html! {
        <div>
            <h3>{"ATIS List"}</h3>
            <table class={classes!("table", "table-striped", "table-bordered", "table-sm")}>
                <thead>
                    <tr>
                        <th >{"Callsign"}</th>
                        <th >{"Letter"}</th>
                        <th >{"ATIS"}</th>
                        <th >{"Alt"}</th>
                        <th >{"Wind"}</th>
                        <th >{"Temp"}</th>
                    </tr>
                </thead>
                <tbody>
                {
                    atis.iter().map(|atis| {
                        html! {
                            <tr>
                                <td>{&atis.callsign}</td>
                                <td>{&atis.information_letter}</td>
                                <td>{&atis.pre_format_atis()}</td>
                                <td>{&atis.altimeter_settings}</td>
                                <td>{&atis.format_wind()}</td>
                                <td>{&atis.temperature}</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
                </tbody>
            </table>
        </div>
    }
}
