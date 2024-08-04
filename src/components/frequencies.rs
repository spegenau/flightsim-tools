use std::cmp::max;

use gloo_console::log;
use yew::{classes, function_component, html, use_context, Html, Properties};

use crate::{vatsim::vatsim_data_manager::VatsimDataManager, Context};

#[derive(PartialEq, Default, Clone)]
pub struct Frequency {
    pub name: String,
    pub frequency: String,
}

#[derive(Properties, PartialEq)]
pub struct FrequenciesProps {
    #[prop_or(Vec::new())]
    pub frequencies: Vec<Frequency>,
    #[prop_or(1)]
    pub total_lines: i32,
    #[prop_or_default]
    pub show_unicom: bool,
    #[prop_or_default]
    pub show_center_stations: bool,

    #[prop_or("50%".to_string())]
    pub name_width: String,
    #[prop_or("50%".to_string())]
    pub freq_width: String,
    #[prop_or("0.7cm".to_string())]
    pub min_line_height: String,
}

#[function_component]
pub fn Frequencies(props: &FrequenciesProps) -> Html {
    let ctx = use_context::<Context>().expect("no ctx found");
    if props.show_center_stations {
        let vatsim_data_manager = VatsimDataManager {
            vatsim: ctx.vatsim,
            transceivers: ctx.transceivers.clone(),
        };
        let center_stations = vatsim_data_manager.get_center_stations(ctx.simbrief.navlog.fix);
    }
    let max = max(props.total_lines, props.frequencies.len() as i32);

    let mut frequencies = (0..max)
        .map(|index| match props.frequencies.get(index as usize) {
            Some(freq) => (*freq).clone(),
            None => Frequency::default(),
        })
        .collect::<Vec<Frequency>>();

    if props.show_unicom {
        frequencies.push(Frequency {
            name: "Unicom".to_string(),
            frequency: "122.8".to_string(),
        });
    }

    let style_name = format!("width: {};", props.name_width);
    let style_freq = format!("width: {};", props.freq_width);
    let line_height = format!("height: {};", props.min_line_height);

    html! {
        <table class={classes!("table", "table-striped", "table-bordered", "table-sm")}>
            <thead>
                <tr>
                    <th style={style_name}></th>
                    <th style={style_freq}>{"Freq"}</th>
                </tr>
            </thead>
            <tbody>
                {
                    frequencies.into_iter().map(|freq| {
                        html!{
                            <tr key={freq.name.clone()} style={line_height.clone()}>
                                <td>{&freq.name}</td>
                                <td>{&freq.frequency}</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
