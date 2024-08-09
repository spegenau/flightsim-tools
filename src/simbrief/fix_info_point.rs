use geo::coord;
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

impl FixInfoPoint {
    pub fn as_lat_long(&self) -> geo::Coord {
        let lat = self.pos_lat.parse::<f64>().unwrap();
        let long = self.pos_long.parse::<f64>().unwrap();

        coord! {x: lat, y: long}
    }
}
