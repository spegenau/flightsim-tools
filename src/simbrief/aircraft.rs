use serde::Deserialize;


#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Aircraft {
    pub icaocode: String,
}