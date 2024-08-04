use std::collections::HashMap;

use geo::{coord, Contains, Coord};
use gloo_console::log;
use web_sys::console::log;

use crate::{
    components::{controllers, frequencies},
    simbrief::fix_info_point::FixInfoPoint,
};

use super::{
    transceiver::{self, Transceiver},
    vatsim_response::VatsimResponse,
};
#[derive(PartialEq, Hash, Eq)]
pub enum ControllerType {
    Approach,
    Atis,
    Control,
    Delivery,
    Ground,
    Observer,
    Supervisor,
    Tower,
    None,
}

impl Default for ControllerType {
    fn default() -> Self {
        ControllerType::None
    }
}

impl ControllerType {
    fn as_str(&self) -> &'static str {
        match self {
            ControllerType::Approach => "APP",
            ControllerType::Atis => "ATIS",
            ControllerType::Control => "CTR",
            ControllerType::Delivery => "DEL",
            ControllerType::Ground => "GND",
            ControllerType::Observer => "OBS",
            ControllerType::Supervisor => "SUP",
            ControllerType::Tower => "TWR",
            ControllerType::None => "INVALID",
        }
    }
}
#[derive(PartialEq, Hash, Default)]
pub struct ControllerLine {
    pub callsign: String,
    pub controller_type: ControllerType,
    pub frequencies: Vec<String>,
}

impl ControllerLine {
    pub fn new(controller_type: ControllerType) -> Self {
        ControllerLine {
            callsign: String::new(),
            controller_type,
            frequencies: Vec::new(),
        }
    }
}

pub struct VatsimDataManager {
    pub vatsim: VatsimResponse,
    pub transceivers: Vec<Transceiver>,
}

impl VatsimDataManager {
    pub fn get_controllers(&self) -> Vec<Transceiver> {
        self.transceivers
            .clone()
            .into_iter()
            .filter(|t| t.callsign.contains('_'))
            .collect()
    }

    pub fn get_center_stations(&self, routing: Vec<FixInfoPoint>) -> Vec<ControllerLine> {
        let controllers: Vec<Transceiver> = self
            .get_controllers()
            .into_iter()
            .filter(|t| t.callsign.ends_with("_CTR"))
            .collect();
        log!("Controllers", controllers.len());

        let mut centers = Vec::new();

        for point in routing {
            let coordinate: Coord<f64> =
                coord! { x: point.pos_lat.parse().unwrap(), y: point.pos_long.parse().unwrap() };

            for controller in controllers.clone() {
                if controller.contains_coord(&coordinate) {
                    log!("Found center station: {}", controller.callsign.clone());
                    let freq = controller.transceivers.first().unwrap().frequency;
                    let frequencies = vec![VatsimDataManager::frequency_to_string(freq)];
                    centers.push(ControllerLine {
                        callsign: controller.callsign,
                        controller_type: ControllerType::Control,
                        frequencies,
                    });
                }
            }
        }

        centers
    }

    pub fn get_stations_for_airport(
        &self,
        airport_callsign: &str,
    ) -> HashMap<ControllerType, ControllerLine> {
        if airport_callsign.is_empty() {
            return HashMap::new();
        }

        let mut stations: HashMap<ControllerType, ControllerLine> = HashMap::from([
            (
                ControllerType::Delivery,
                ControllerLine::new(ControllerType::Delivery),
            ),
            (
                ControllerType::Ground,
                ControllerLine::new(ControllerType::Ground),
            ),
            (
                ControllerType::Tower,
                ControllerLine::new(ControllerType::Tower),
            ),
            (
                ControllerType::Approach,
                ControllerLine::new(ControllerType::Approach),
            ),
        ]);

        self.get_controllers()
            .into_iter()
            .filter(|t| t.callsign.starts_with(airport_callsign))
            .for_each(|t| {
                //log!("Transceiver: {:?}", t.callsign.clone());
                for (controller_type, line) in stations.iter_mut() {
                    if t.callsign.ends_with(controller_type.as_str()) {
                        line.callsign.push_str(&t.callsign);
                        for transceiver in &t.transceivers {
                            line.frequencies
                                .push(VatsimDataManager::frequency_to_string(
                                    transceiver.frequency,
                                ));
                        }
                        line.frequencies.dedup();
                    }
                }
            });

        stations
    }

    fn frequency_to_string(freq: u32) -> String {
        let freq = freq / 1000;
        let freq: f32 = (freq as f32) / 1000.0;
        format!("{}", freq)
    }
}
