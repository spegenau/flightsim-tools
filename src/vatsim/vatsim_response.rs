use super::{atis::Atis, controller::Controller, parsed_atis::ParsedAtis};
use gloo_console::log;
use serde::Deserialize;
use std::collections::HashMap;

pub const VATSIM_URL: &str = "https://data.vatsim.net/v3/vatsim-data.json";

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct VatsimResponse {
    pub controllers: Vec<Controller>,
    pub atis: Vec<Atis>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AtisType {
    Departure,
    Arrival,
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

    pub fn get_all_controllers_as_map(&self) -> HashMap<String, Controller> {
        self.get_all_controllers()
            .into_iter()
            .map(|c| (c.callsign.clone(), c.clone()))
            .collect()
    }

    pub fn get_parsed_atis_list(&self) -> Vec<ParsedAtis> {
        self.atis
            .clone()
            .into_iter()
            .filter(|a| a.text_atis.is_some())
            .map(ParsedAtis::from)
            .collect()
    }

    pub fn get_atis_for_callsign(&self, callsign: &str) -> Option<ParsedAtis> {
        if callsign.is_empty() {
            return None;
        }
        self.get_parsed_atis_list()
            .clone()
            .iter()
            .find(|a| a.callsign == callsign)
            .cloned()
    }

    pub fn get_atis_for_airport(&self, airport: &str, atis_type: AtisType) -> Option<ParsedAtis> {
        if airport.is_empty() {
            return None;
        }

        let atis_for_airport: Vec<ParsedAtis> = self
            .get_parsed_atis_list()
            .iter()
            .filter(|a| a.callsign.starts_with(airport))
            .cloned()
            .collect();

        if atis_for_airport.len() == 1 {
            let atis: ParsedAtis = atis_for_airport.first().unwrap().clone();

            return Some(atis);
        } else if atis_for_airport.len() == 2 {
            let modifier = if atis_type == AtisType::Departure {
                "_D_"
            } else {
                "_A_"
            };

            let atis = atis_for_airport
                .iter()
                .find(|atis| atis.callsign.contains(modifier))
                .cloned();

            return atis;
        } else if atis_for_airport.len() > 2 {
            log!("Found more atis messages than expected");
            for atis in atis_for_airport.iter() {
                log!(atis.to_js_value())
            }
        }

        None
    }
}
