use super::controller::Controller;
use serde::Deserialize;
use std::collections::HashMap;

pub const VATSIM_URL: &str = "https://data.vatsim.net/v3/vatsim-data.json";

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct VatsimResponse {
    pub controllers: Vec<Controller>,
}

impl VatsimResponse {
    pub fn get_all_controllers(&self) -> Vec<Controller> {
        self.controllers
            .clone()
            .into_iter()
            .filter(|c| c.callsign.contains('_'))
            .filter(|c| c.frequency != "199.998")
            .collect()
    }

    pub fn get_controllers_by_callsign(&self, callsign: &str) -> HashMap<String, Controller> {
        if callsign.is_empty() {
            return HashMap::new();
        }

        self.get_all_controllers()
            .into_iter()
            .filter(|c| c.callsign.starts_with(callsign))
            .map(|c| (c.callsign.clone(), c.clone()))
            .collect()
    }
}
