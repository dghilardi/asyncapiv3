use std::collections::HashMap;

pub type Operations = HashMap<String, Operation>;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {

}