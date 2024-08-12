use super::atis::Atis;
use regex::Regex;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ParsedAtis {
    pub name: String,
    pub callsign: String,
    pub raw_data: Vec<String>,
    pub one_line_data: String,

    pub information_letter: String,

    pub altimeter_settings: String,
    pub wind_dir: String,
    pub wind_kt: String,
    pub wind_gust: String,
    pub temperature: String,
}

impl ParsedAtis {
    pub fn from(atis: Atis) -> Self {
        let one_line_data = atis.text_atis.clone().unwrap_or_default().join(" ");

        let altimeter_settings = ParsedAtis::parse_altimeter_settings(&one_line_data);
        let temperature = ParsedAtis::parse_temperature(&one_line_data);

        Self {
            name: atis.name,
            callsign: atis.callsign,
            raw_data: atis.text_atis.clone().unwrap_or_default(),
            one_line_data,
            information_letter: atis.atis_code.unwrap_or_default(),

            altimeter_settings,
            wind_dir: String::new(),
            wind_kt: String::new(),
            wind_gust: String::new(),
            temperature,
        }
    }

    pub fn parse_temperature(atis: &str) -> String {
        let re_temperature = Regex::new(r"\[?TEMPERATURE\]?\s(?P<TEMP>[0-9]{1,2})").unwrap();
        if let Some(caps) = re_temperature.captures(atis) {
            return String::from(&caps["TEMP"]);
        }

        let re_temp = Regex::new(r"\[?T[E]?MP\]?\s(?P<TEMP>[0-9]{1,2})[\.]?").unwrap();
        if let Some(caps) = re_temp.captures(atis) {
            return String::from(&caps["TEMP"]);
        }

        let re_temp = Regex::new(r"(?P<TEMP>[0-9]{1,2})/[0-9]{1,2}").unwrap();
        if let Some(caps) = re_temp.captures(atis) {
            return String::from(&caps["TEMP"]);
        }

        let re_temp = Regex::new(r"T\s(?P<TEMP>[0-9]{1,2})\sDP\s[0-9]{1,2}").unwrap();
        if let Some(caps) = re_temp.captures(atis) {
            return String::from(&caps["TEMP"]);
        }
        String::new()
    }

    pub fn parse_altimeter_settings(atis: &str) -> String {
        let re_qnh = Regex::new(r"\[?QNH\]?\s(?P<ALT>[0-9]{3,4})").unwrap();
        if let Some(caps) = re_qnh.captures(atis) {
            return String::from(&caps["ALT"]);
        }

        let re_q = Regex::new(r"Q(?P<ALT>[0-9]{3,4})").unwrap();
        if let Some(caps) = re_q.captures(atis) {
            return String::from(&caps["ALT"]);
        }

        let re_altimeter = Regex::new(r"ALTIMETER\s(?P<ALT>[0-9]{4})").unwrap();
        if let Some(caps) = re_altimeter.captures(atis) {
            let setting = String::from(&caps["ALT"]);
            let setting = setting.parse::<f32>().unwrap();
            let setting = format!("{0:.2}", setting / 100.0);
            return setting;
        }

        let re_a = Regex::new(r"A(?P<ALT>[0-9]{4})").unwrap();
        if let Some(caps) = re_a.captures(atis) {
            return String::from(&caps["ALT"]);
        }

        String::new()
    }

    pub fn pre_format_atis(&self) -> String {
        self.raw_data.join("\n")
    }

    pub fn format_wind(&self) -> String {
        format!("{}/{}KT/G{}", self.wind_dir, self.wind_kt, self.wind_gust)
    }
}
