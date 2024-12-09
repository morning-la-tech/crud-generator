use super::field::Field;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Relation {
    pub r#type: String,
    pub related_entity: String,
    pub from_column: Option<String>,
    pub to_column: Option<String>,
    pub pivot_table: Option<String>,
    pub pivot_table_fields: Option<Vec<Field>>,
}
