use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ViewCollection {
    pub offset: Option<u32>,
    pub rows: Vec<ViewItem>,
    pub total_rows: u32,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ViewItem {
    pub id: String,
    pub key: String,
    pub value: String,
}