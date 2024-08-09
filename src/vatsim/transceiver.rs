use std::collections::HashMap;

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

    pub fn get_frequency(&self) -> Vec<String> {
        let mut frequencies: Vec<String> = self
            .transceivers
            .iter()
            .map(|t| t.frequency as f32 / 1000000.0)
            .map(|f| format!("{0:.3}", f))
            .collect();
        frequencies.dedup();

        frequencies
    }

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
                transceiver.latDeg.unwrap_or(0.0),
                transceiver.lonDeg.unwrap_or(0.0),
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
