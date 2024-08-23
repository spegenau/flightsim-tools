use std::collections::HashMap;

use super::transceiver::Transceiver;
#[derive(PartialEq, Hash, Eq)]
pub enum ControllerType {
    Approach,
    // Atis,
    //Control,
    Delivery,
    Ground,
    // Observer,
    // Supervisor,
    Tower,
    None,
}

impl ControllerType {
    fn as_str(&self) -> &'static str {
        match self {
            ControllerType::Approach => "APP",
            // ControllerType::Atis => "ATIS",
            //ControllerType::Control => "CTR",
            ControllerType::Delivery => "DEL",
            ControllerType::Ground => "GND",
            // ControllerType::Observer => "OBS",
            // ControllerType::Supervisor => "SUP",
            ControllerType::Tower => "TWR",
            ControllerType::None => "INVALID",
        }
    }
}
#[derive(PartialEq, Hash)]
pub struct ControllerLine {
    pub callsign: String,
    pub controller_type: ControllerType,
    pub frequencies: Vec<String>,
}

impl Default for ControllerLine {
    fn default() -> Self {
        Self {
            callsign: Default::default(),
            controller_type: ControllerType::None,
            frequencies: Default::default(),
        }
    }
}

impl ControllerLine {
    pub fn new(controller_type: ControllerType) -> Self {
        ControllerLine {
            callsign: String::new(),
            controller_type,
            frequencies: Vec::new(),
        }
    }

    pub fn format_frequencies(&self) -> String {
        let mut frequencies = self.frequencies.clone();
        frequencies.sort();
        frequencies.dedup();

        frequencies.join(", ")
    }
}

pub struct VatsimDataManager {
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
                        line.frequencies.sort();
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
