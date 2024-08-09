use std::cmp::max;

use gloo_console::log;
use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Default, Clone)]
pub struct Frequency {
    pub id: u8,
    pub callsign: String,
    pub frequency: String,
}

#[derive(Properties, PartialEq)]
pub struct FrequenciesProps {
    #[prop_or(Vec::new())]
    pub frequencies: Vec<Frequency>,
    #[prop_or(1)]
    pub total_lines: u8,
    #[prop_or_default]
    pub show_unicom: bool,

    #[prop_or("50%".to_string())]
    pub name_width: String,
    #[prop_or("50%".to_string())]
    pub freq_width: String,
    #[prop_or("0.7cm".to_string())]
    pub min_line_height: String,
}

#[function_component]
pub fn Frequencies(props: &FrequenciesProps) -> Html {
    let mut frequencies = props.frequencies.clone();
    let max = max(props.total_lines, props.frequencies.len() as u8);

    while frequencies.len() < max as usize {
        frequencies.push(Frequency::default())
    }

    if props.show_unicom {
        frequencies.push(Frequency {
            id: 0,
            callsign: "Unicom".to_string(),
            frequency: "122.8".to_string(),
        });
    }

    for freq in &frequencies {
        log!(format!(
            "{} - {} - {}",
            freq.id,
            freq.callsign.clone(),
            freq.frequency.clone()
        ));
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
                            <tr key={freq.id} style={line_height.clone()}>
                                <td>{&freq.callsign}</td>
                                <td>{&freq.frequency}</td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    }
}
