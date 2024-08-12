use serde::Deserialize;
#[derive(Clone, PartialEq, Deserialize, Debug, Default)]

pub struct Atis {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub atis_code: Option<String>,
    pub text_atis: Option<Vec<String>>,
}
