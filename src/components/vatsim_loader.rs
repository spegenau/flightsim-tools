use std::rc::Rc;

use gloo::{net::http::Request, timers::callback::Interval};
use yew::{
    function_component, html, use_effect, use_reducer, Callback, Html, Properties, Reducible,
};

use crate::vatsim::vatsim_response::{VatsimResponse, VATSIM_URL};

pub const UPDATE_EVERY_X_SECONDS: u32 = 60;

#[derive(Properties, PartialEq)]
pub struct VatsimLoaderProps {
    pub on_vatsim_update: Callback<VatsimResponse>,
    pub on_error: Callback<Vec<String>>,
}

enum SecondsStateAction {
    Increment,
}

#[derive(Default)]
struct SecondsState {
    seconds: usize,
}

impl Reducible for SecondsState {
    /// Reducer Action Type
    type Action = SecondsStateAction;

    /// Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SecondsStateAction::Increment => Self {
                seconds: self.seconds + 1,
            }
            .into(),
        }
    }
}

#[function_component]
pub fn VatsimLoader(props: &VatsimLoaderProps) -> Html {
    let seconds_state_handle = use_reducer(SecondsState::default);

    use_effect({
        let seconds_state_handle = seconds_state_handle.clone();

        let event = props.on_vatsim_update.clone();
        let error_handler = props.on_error.clone();
        move || {
            // i intervals get out of scope they get dropped and destroyed
            let interval = Interval::new(1000, move || {
                let seconds = seconds_state_handle.seconds;

                if seconds % UPDATE_EVERY_X_SECONDS as usize == 0 {
                    let value = event.clone();
                    let error_handler = error_handler.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let error_handler = error_handler.clone();
                        match Request::get(VATSIM_URL).send().await {
                            Ok(response) => {
                                let json_result = response.json::<VatsimResponse>().await;
                                match json_result {
                                    Ok(vatsim) => {
                                        value.emit(vatsim);
                                    }
                                    Err(e) => {
                                        error_handler.emit(vec![format!(
                                            "Error parsing VATSIM data: {}",
                                            e
                                        )]);
                                    }
                                }
                            }
                            Err(e) => {
                                error_handler
                                    .emit(vec![format!("Error fetching VATSIM data: {}", e)]);
                            }
                        }
                    });
                }

                seconds_state_handle.dispatch(SecondsStateAction::Increment)
            });

            // So we move it into the clean up function, rust will consider this still being used and wont drop it
            // then we just drop it ourselves in the cleanup
            move || drop(interval)
        }
    });

    html! {
        <></>
    }
}
