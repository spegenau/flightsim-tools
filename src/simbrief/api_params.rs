use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct ApiParams {
    pub origrwy: String,
    pub destrwy: String,
}
