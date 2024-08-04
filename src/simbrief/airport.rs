use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Airport {
    pub icao_code: String,
    pub name: String,
    pub plan_rwy: String,
}

impl Airport {
    pub fn format_name(&self) -> String {
        let mut name = self.icao_code.clone();

        if !self.name.is_empty() {
            name.push_str(" (");
            name.push_str(self.name.as_str());
            name.push(')');
        }

        name
    }
}
