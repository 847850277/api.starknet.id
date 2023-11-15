use mongodb::Database;

use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AppState {
    pub conf: Config,
    pub starknetid_db: Database,
    pub sales_db: Database,
    pub states: States,
}

#[derive(Serialize)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_expiry: Option<i64>,
    pub is_owner_main: bool,
    pub owner_addr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_github: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_discord: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_personhood: Option<bool>,
    pub starknet_id: String,
    pub img_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct State {
    pub rate: f32,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Debug)]
pub struct States {
    pub states: HashMap<String, State>,
}
