use serde::Deserialize;

use super::{
    aircraft::Aircraft, airport::Airport, api_params::ApiParams, general::General, navlog::Navlog,
};

pub fn get_simbrief_url(userid: &str) -> String {
    format!("https://www.simbrief.com/api/xml.fetcher.php?userid={userid}&json=1")
}

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct SimbriefResponse {
    pub general: General,
    pub origin: Airport,
    pub destination: Airport,
    pub aircraft: Aircraft,
    pub navlog: Navlog,
    pub api_params: ApiParams,
}
