use serde::Deserialize;

use super::string_or_map::StringOrMap;

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]

pub struct General {
    pub icao_airline: StringOrMap,
    pub flight_number: StringOrMap,
    pub route: String,
}

impl General {
    pub fn get_call_sign(&self) -> String {
        let icao_airline: String = match &self.icao_airline {
            StringOrMap::String(icao_airline) => icao_airline.clone(),
            StringOrMap::HashMap(_) => String::new(),
        };

        let flight_number: String = match &self.flight_number {
            StringOrMap::String(flight_number) => flight_number.clone(),
            StringOrMap::HashMap(_) => String::new(),
        };

        format!("{icao_airline} {flight_number}")
    }

    pub fn get_sid(&self) -> String {
        self.route.split(' ').next().unwrap_or_default().to_string()
    }

    pub fn get_star(&self) -> String {
        self.route
            .split(' ')
            .rev()
            .next()
            .unwrap_or_default()
            .to_string()
    }
}
