use crate::models::Resource;
use std::fs::create_dir_all;
use std::path::Path;
use tera::{Context, Tera};

pub fn generate_migration(tera: &mut Tera, resource: &Resource) {
    let migration_dir = "./migration/src";

    if !Path::new(migration_dir).exists() {
        create_dir_all(migration_dir).unwrap();
    }

    let empty_fields = vec![];
    let fields = resource.fields.as_ref().unwrap_or(&empty_fields);

    // Transformer les champs en tuples comme attendu par le template
    let migration_fields: Vec<_> = fields
        .iter()
        .map(|field| {
            (
                field.name.clone(),
                map_field_type(&field.field_type),
                field.primary_key.unwrap_or(false),
                field.unique.unwrap_or(false),
                field.nullable.unwrap_or(false),
            )
        })
        .collect();

    let mut context = Context::new();
    context.insert("resource_name", &resource.resource_name);
    context.insert("fields", &migration_fields);

    super::files::generate_file(
        tera,
        "migration.rs.tera",
        "migration/src",
        &resource.resource_name,
        &context,
    );
}

fn map_field_type(field_type: &str) -> &str {
    match field_type {
        "String" => "String",
        "Uuid" => "uuid",
        "i32" => "Integer",
        "DateTime<Utc>" => "timestamp_with_time_zone",
        "bool" => "Boolean",
        _ => "String",
    }
}
