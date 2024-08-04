use serde::Deserialize;

use super::fix_info_point::FixInfoPoint;


#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Navlog {
    pub fix: Vec<FixInfoPoint>,
}