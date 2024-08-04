use geo::{Contains, LineString, Polygon};
use serde::Deserialize;

pub const VATSIM_TRANSCEIVER_URL: &str = "https://data.vatsim.net/v3/transceivers-data.json";

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct TransceiverDetails {
    pub id: u32,
    pub frequency: u32,
    pub latDeg: Option<f64>,
    pub lonDeg: Option<f64>,
}

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Transceiver {
    pub callsign: String,
    pub transceivers: Vec<TransceiverDetails>,

    pub polygon: Option<Polygon>,
}

impl Transceiver {
    pub fn generate_polygon(&mut self) {
        let points: Vec<(f64, f64)> = self
            .transceivers
            .iter()
            .map(|t| (t.latDeg.unwrap_or(0.0), t.lonDeg.unwrap_or(0.0)))
            .collect();

        let polygon = Polygon::new(LineString::from(points), vec![]);

        self.polygon = Some(polygon);
    }

    pub fn contains_coord(&self, coordinate: &geo::Coord<f64>) -> bool {
        let points: Vec<(f64, f64)> = self
            .transceivers
            .iter()
            .map(|t| (t.latDeg.unwrap_or(0.0), t.lonDeg.unwrap_or(0.0)))
            .collect();

        let polygon = Polygon::new(LineString::from(points), vec![]);

        polygon.contains(coordinate)
    }
}
