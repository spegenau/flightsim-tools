use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlSelectElement};
use yew::Event;
use yew::{
    classes, function_component, html, use_context, use_state, AttrValue, Callback, Html,
    InputEvent,
};
use yew_bootstrap::component::form::*;
use yew_bootstrap::component::*;

use crate::vatsim::{parsed_atis::ParsedAtis, vatsim_response::VatsimResponse};

pub const RADIO_NO_FILTER: &str = "radio_no_filter";
pub const RADIO_NO_TEMP_FILTER: &str = "radio_no_temp_filter";
pub const RADIO_NO_WIND_FILTER: &str = "radio_no_wind_filter";
pub const RADIO_NO_ALT_FILTER: &str = "radio_no_alt_filter";
pub const RADIO_NO_TL_FILTER: &str = "radio_no_tl_filter";

#[function_component]
pub fn AtisList() -> Html {
    let vatsim = use_context::<VatsimResponse>().expect("no ctx found");

    let filter_setting = use_state(|| AttrValue::from(RADIO_NO_FILTER));
    let search_term = use_state(AttrValue::default);

    let oninput = Callback::from({
        let search_term = search_term.clone();
        move |event: InputEvent| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            match input {
                Some(input) => {
                    log!("oninput event", input.value());
                    search_term.set(AttrValue::from(input.value()));
                }
                None => todo!(),
            }
        }
    });

    let onchange = Callback::from({
        let filter_setting = filter_setting.clone();
        move |event: Event| {
            let target: Option<EventTarget> = event.target();

            // Input element
            let input = target
                .clone()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let name = input.name();
                let value = input.value();
                let id = input.id();
                log!("onchange for HtmlInputElement ", &name, &id, value);
            }

            let input = target
                .clone()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                let name = input.name();
                let value = input.value();
                let id = input.id();
                log!("onchange for HtmlSelectElement ", name, id, &value);
                filter_setting.set(AttrValue::from(value));
            }
        }
    });

    let atis_all: Vec<ParsedAtis> = vatsim.get_parsed_atis_list().clone();
    let atis_missing_alt: Vec<ParsedAtis> = atis_all
        .iter()
        .filter(|atis| atis.misses_alt())
        .cloned()
        .collect();
    let atis_missing_wind: Vec<ParsedAtis> = atis_all
        .iter()
        .filter(|atis| atis.misses_wind())
        .cloned()
        .collect();
    let atis_missing_temp: Vec<ParsedAtis> = atis_all
        .iter()
        .filter(|atis| atis.misses_temp())
        .cloned()
        .collect();
    let atis_missing_tl: Vec<ParsedAtis> = atis_all
        .iter()
        .filter(|atis| atis.misses_transition_level())
        .cloned()
        .collect();

    let atis = match filter_setting.clone().as_str() {
        RADIO_NO_ALT_FILTER => atis_missing_alt.clone(),
        RADIO_NO_WIND_FILTER => atis_missing_wind.clone(),
        RADIO_NO_TEMP_FILTER => atis_missing_temp.clone(),
        RADIO_NO_TL_FILTER => atis_missing_tl.clone(),
        _ => atis_all.clone(),
    };

    let search_term_string = search_term.clone().to_string();
    let atis = if !search_term.is_empty() {
        atis.iter()
            .filter(|atis| {
                atis.callsign
                    .to_lowercase()
                    .contains(&search_term_string.to_lowercase())
            })
            .cloned()
            .collect()
    } else {
        atis
    };

    html! {
        <div>
            <Row class="mb-1">
                <Column>
                    <FormControl id="input-text" ctype={FormControlType::Text} label="Callsign" oninput={ oninput.clone() } value={AttrValue::from(search_term_string.clone())}/>
                </Column>
                <Column>
                    <FormControl
                            id="filter"
                            name="filter"
                            ctype={ FormControlType::Select}
                            label={ "Filter" }
                            onchange={ onchange.clone() }
                        >
                            <SelectOption key=0 label={format!("All ({})", atis_all.len())} value={RADIO_NO_FILTER} selected={ *filter_setting.clone() == RADIO_NO_FILTER } />
                            <SelectOption key=1 label={format!("Missing ALT ({})", atis_missing_alt.len())} value={RADIO_NO_ALT_FILTER} selected={ *filter_setting.clone() == RADIO_NO_ALT_FILTER }/>
                            <SelectOption key=2 label={format!("Missing Wind ({})", atis_missing_wind.len())} value={RADIO_NO_WIND_FILTER} selected={ *filter_setting.clone() == RADIO_NO_WIND_FILTER }/>
                            <SelectOption key=3 label={format!("Missing Temp ({})", atis_missing_temp.len())} value={RADIO_NO_TEMP_FILTER} selected={ *filter_setting.clone() == RADIO_NO_TEMP_FILTER }/>
                            <SelectOption key=4 label={format!("Missing TL ({})", atis_missing_tl.len())} value={RADIO_NO_TL_FILTER} selected={ *filter_setting.clone() == RADIO_NO_TL_FILTER }/>
                        </FormControl>
                </Column>
            </Row>
            <Row>
                <Column>
                    <table class={classes!("table", "table-striped", "table-bordered", "table-sm")}>
                        <thead>
                            <tr>
                                <th >{"Callsign"}</th>
                                <th >{"Letter"}</th>
                                <th >{"ATIS"}</th>
                                <th >{"Alt"}</th>
                                <th >{"Wind"}</th>
                                <th >{"Temp"}</th>
                                <th >{"TL"}</th>
                            </tr>
                        </thead>
                        <tbody>
                        {
                            atis.iter().map(|atis| {
                                html! {
                                    <tr key={atis.callsign.clone()}>
                                        <td>{&atis.callsign}</td>
                                        <td>{&atis.information_letter}</td>
                                        <td>{&atis.pre_format_atis()}</td>
                                        <td>{&atis.altimeter_settings}</td>
                                        <td>{&atis.format_wind()}</td>
                                        <td>{&atis.temperature}</td>
                                        <td>{&atis.transition_level}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                        </tbody>
                    </table>
                </Column>
            </Row>
        </div>
    }
}
