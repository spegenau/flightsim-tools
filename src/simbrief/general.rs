use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct General {
    pub icao_airline: String,
    pub flight_number: String,
    pub route: String,
}

impl General {
    pub fn get_call_sign(&self) -> String {
        let mut call_sign = self.icao_airline.clone();
        call_sign.push(' ');
        call_sign.push_str(self.flight_number.as_str());

        call_sign
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
