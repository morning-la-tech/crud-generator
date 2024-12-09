use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Field {
    pub name: String,
    pub field_type: String,
    pub primary_key: Option<bool>,
    pub unique: Option<bool>,
    pub nullable: Option<bool>,
}
