use gloo::storage::LocalStorage;
use gloo_net::http::Request;
use gloo_storage::Storage;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{classes, function_component, html, use_state, Callback, Html, InputEvent, Properties};

use crate::simbrief::simbrief_response::{get_simbrief_url, SimbriefResponse};

#[derive(Properties, PartialEq)]
pub struct SimbriefLoaderProps {
    pub on_simbrief_update: Callback<SimbriefResponse>,
}

#[function_component]
pub fn SimbriefLoader(props: &SimbriefLoaderProps) -> Html {
    // Try to fetch simbrief data from local storage
    let simbrief_id_local_storage: String = LocalStorage::get("simbrief_id").unwrap_or_default();

    let is_loading = use_state(bool::default);
    let has_loaded = use_state(bool::default);
    let simbrief_id = use_state(|| simbrief_id_local_storage);
    let value = simbrief_id.clone();

    let oninput = Callback::from({
        let simbrief_id = simbrief_id.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            //web_sys::console::log_1(&target.value().into()); // <- can console the value.
            let id = target.value();
            simbrief_id.set(id.clone());
            LocalStorage::set("simbrief_id", id).ok();
        }
    });

    let mut btn_classes: Vec<String> = vec!["btn".to_string()];
    if *(has_loaded.clone()) {
        btn_classes.push("btn-success".to_string());
    } else if *(is_loading.clone()) {
        btn_classes.push("btn-secondary".to_string());
    } else {
        btn_classes.push("btn-primary".to_string());
    }

    let button_text = if *(has_loaded.clone()) {
        "Refresh".to_string()
    } else if *(is_loading.clone()) {
        "Is Loading...".to_string()
    } else {
        "Load".to_string()
    };

    let event = props.on_simbrief_update.clone();
    let onclick = Callback::from(move |_| {
        // Set loading flag
        let is_loading = is_loading.clone();
        is_loading.set(true);
        let has_loaded = has_loaded.clone();

        // Get the url
        let simbrief_id = simbrief_id.clone();
        let url = get_simbrief_url(&simbrief_id);

        let event = event.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let simbrief: SimbriefResponse = Request::get(&url)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            event.emit(simbrief);
            is_loading.set(false);
            has_loaded.set(true);
        });
    });

    html! {
        <div>
            <div class={classes!("input-group", "pr-3")}>
                <input type={"text"} class={classes!("form-control")} placeholder={"Simbrief Id"} aria-label={"Simbrief Id"} value={<std::string::String as Clone>::clone(&*(value.clone()))} {oninput}/>
                <button {onclick} type="button" class={classes!(btn_classes)}>{button_text}</button>
            </div>
        </div>
    }
}
