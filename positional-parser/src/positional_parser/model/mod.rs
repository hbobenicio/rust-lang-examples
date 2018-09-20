use std::collections;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaInfo {
    // pub fields: collections::HashMap<String, Field>
    pub fields: Vec<collections::HashMap<String, Field>>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub size: usize,

    #[serde(rename = "type")]
    pub typ: String
}
