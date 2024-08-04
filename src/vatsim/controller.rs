use serde::Deserialize;
#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Controller {
    pub cid: u32,
    pub name: String,
    pub callsign: String,
    pub frequency: String,
    pub facility: i32,
    pub rating: i32,
    pub visual_range: i32,
}
