use super::{field::Field, relation::Relation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Resource {
    pub resource_name: String,
    pub fields: Option<Vec<Field>>,
    pub relations: Option<Vec<Relation>>,
}
