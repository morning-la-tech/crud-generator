use crate::models::{Relation, Resource};
use tera::{Context, Tera};

pub fn generate_pivot_file(tera: &Tera, input: &Resource, relation: &Relation) {
    if let Some(pivot_table) = &relation.pivot_table {
        let mut pivot_context = Context::new();
        pivot_context.insert("pivot_table_name", pivot_table);
        pivot_context.insert("fields", &relation.pivot_table_fields);
        pivot_context.insert("entity_1", &input.resource_name);
        pivot_context.insert("entity_2", &relation.related_entity);

        super::files::generate_file(
            tera,
            "pivot_model.rs.tera",
            "models",
            pivot_table,
            &pivot_context,
        );
    }
}
