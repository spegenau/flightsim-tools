use std::collections::HashMap;

use geo::{Contains, LineString, Polygon};
use serde::Deserialize;

pub const VATSIM_TRANSCEIVER_URL: &str = "https://data.vatsim.net/v3/transceivers-data.json";

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct TransceiverDetails {
    pub id: u32,
    pub frequency: u32,
    #[serde(rename = "latDeg")]
    pub lat_deg: Option<f64>,
    #[serde(rename = "lonDeg")]
    pub lon_deg: Option<f64>,
}

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Transceiver {
    pub callsign: String,
    pub transceivers: Vec<TransceiverDetails>,

    pub polygon: Option<Polygon>,
}

impl Transceiver {
    pub fn get_callsign_and_frequency(
        &self,
        coordinate: &geo::Coord<f64>,
    ) -> Option<(String, String)> {
        for (polygon, frequency) in self.get_polygon_to_frequency() {
            if polygon.contains(coordinate) {
                return Some((self.callsign.clone(), frequency));
            }
        }
        None
    }

    fn get_polygon_to_frequency(&self) -> Vec<(Polygon, String)> {
        let mut frequency_to_points: HashMap<String, Vec<(f64, f64)>> = HashMap::new();

        for transceiver in &self.transceivers {
            let point = (
                transceiver.lat_deg.unwrap_or(0.0),
                transceiver.lon_deg.unwrap_or(0.0),
            );

            let frequency = transceiver.frequency as f32 / 1000000.0;
            let frequency = format!("{0:.3}", frequency);

            if let std::collections::hash_map::Entry::Vacant(e) =
                frequency_to_points.entry(frequency.clone())
            {
                e.insert(vec![point]);
            } else {
                frequency_to_points.get_mut(&frequency).unwrap().push(point);
            }
        }

        let mut polygons = Vec::new();
        for (frequency, points) in frequency_to_points {
            let polygon = Polygon::new(LineString::from(points), vec![]);
            polygons.push((polygon, frequency));
        }

        polygons
    }
}
