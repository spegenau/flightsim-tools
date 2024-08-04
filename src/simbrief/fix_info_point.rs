use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct FixInfoPoint {
    pub ident: String,
    pub name: String,
    pub pos_lat: String,
    pub pos_long: String,
    pub via_airway: String,
    pub is_sid_star: String,
}
