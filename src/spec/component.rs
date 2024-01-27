use std::collections::HashMap;

pub type Components = HashMap<String, Component>;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {

}