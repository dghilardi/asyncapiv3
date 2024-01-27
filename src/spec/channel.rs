use std::collections::HashMap;

pub type Channels = HashMap<String, Channel>;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {

}