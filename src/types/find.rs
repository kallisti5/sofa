use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

/// Sort direction abstraction
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum SortDirection {
    Desc,
    Asc,
}

impl From<String> for SortDirection {
    fn from(original: String) -> SortDirection {
        match original.as_ref() {
            "desc" => SortDirection::Desc,
            "asc" | _ => SortDirection::Asc
        }
    }
}

/// Sort spec content abstraction
pub type SortSpecContent = HashMap<String, String>;

/// Sort spec abstraction
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum SortSpec {
    Simple(String),
    Complex(SortSpecContent),
}

/// Index spec abstraction
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum IndexSpec {
    DesignDocument(String),
    IndexName((String, String)),
}

/// Find query abstraction
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FindQuery {
    pub selector: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortSpec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_index: Option<IndexSpec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark: Option<String>,
}

/// Find result abstraction
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct FindResult {
    pub docs: Option<Vec<Value>>,
    pub warning: Option<String>,
    pub error: Option<String>,
    pub reason: Option<String>,
    pub bookmark: Option<String>,
}

//todo: include status on structs

/// Explain result abstraction
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExplainResult {
    pub dbname: String,
    pub index: IndexSpec,
    pub selector: Value,
    pub opts: Value,
    pub limit: u32,
    pub skip: u64,
    pub fields: Vec<String>,
    pub range: Value,
}

/// Returns all documents
#[macro_export]
macro_rules! find_all_selector {
        () => {
            json!({"selector" : { "_id" : {"$ne": "null"}}})
        }
    }

impl FindQuery {
    pub fn find_all() -> FindQuery {
        FindQuery {
            selector: json!({ "_id" : {"$ne": null}}),
            limit: None,
            skip: None,
            sort: None,
            fields: None,
            use_index: None,
            bookmark: None,
        }
    }
}