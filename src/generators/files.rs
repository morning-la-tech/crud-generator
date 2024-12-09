use crate::models::Resource;
use crate::utils::file::append_to_mod_file;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

pub fn generate_files(tera: &Tera, input: &Resource) {
    let mut context = Context::new();
    context.insert("resource_name", &input.resource_name);

    // Gérer les champs vides
    let empty_fields = vec![];
    let fields = input.fields.as_ref().unwrap_or(&empty_fields);
    context.insert("fields", fields);

    // Gérer les relations vides
    let empty_relations = vec![];
    let relations = input.relations.as_ref().unwrap_or(&empty_relations);
    context.insert("relations", relations);

    let base_folders = vec!["models", "handlers", "repositories", "managers", "payloads"];

    for folder in &base_folders {
        let folder_path = format!("./src/{}", folder);
        if !Path::new(&folder_path).exists() {
            create_dir_all(&folder_path).unwrap();
        }
    }

    // Create tests directory if it doesn't exist
    let tests_path = format!("./tests/{}", input.resource_name.to_lowercase());
    if !Path::new(&tests_path).exists() {
        create_dir_all(&tests_path).unwrap();
    }

    generate_file(
        tera,
        "model.rs.tera",
        "models",
        &input.resource_name,
        &context,
    );
    generate_file(
        tera,
        "handler.rs.tera",
        "handlers",
        &input.resource_name,
        &context,
    );
    generate_file(
        tera,
        "repository.rs.tera",
        "repositories",
        &input.resource_name,
        &context,
    );
    generate_file(
        tera,
        "manager.rs.tera",
        "managers",
        &input.resource_name,
        &context,
    );
    generate_file(
        tera,
        "payloads.rs.tera",
        "payloads",
        &input.resource_name,
        &context,
    );

    // Generate test files
    generate_test_files(tera, &input.resource_name, &context);
}

fn generate_test_files(tera: &Tera, resource_name: &str, context: &Context) {
    // Generate the test file
    let rendered = tera.render("test.rs.tera", context).unwrap();
    let test_path = format!("./tests/{}", resource_name.to_lowercase());
    let test_file_path = format!("{}/crud_{}.rs", test_path, resource_name.to_lowercase());

    let mut file = File::create(&test_file_path).unwrap();
    file.write_all(rendered.as_bytes()).unwrap();

    // Create or update mod.rs in the resource test directory
    let mod_path = format!("{}/mod.rs", test_path);
    let mut mod_file = File::create(&mod_path).unwrap();
    mod_file
        .write_all(format!("mod crud_{};", resource_name.to_lowercase()).as_bytes())
        .unwrap();

    // Update main tests/mod.rs if needed
    let main_mod_path = "./tests/mod.rs";
    if !Path::new(main_mod_path).exists() {
        let mut main_mod_file = File::create(main_mod_path).unwrap();
        main_mod_file.write_all(b"pub mod common;\n").unwrap();
    }

    append_to_mod_file("tests", resource_name, main_mod_path);
}

pub fn generate_file(
    tera: &Tera,
    template: &str,
    folder: &str,
    resource_name: &str,
    context: &Context,
) {
    let file_name = match folder {
        "models" => resource_name,
        _ => &format!("{}_{}", resource_name, template.split('.').next().unwrap()),
    };

    let template_content = match template {
        "model.rs.tera" => include_str!("../../templates/model.rs.tera"),
        "repository.rs.tera" => include_str!("../../templates/repository.rs.tera"),
        "handler.rs.tera" => include_str!("../../templates/handler.rs.tera"),
        "migration.rs.tera" => include_str!("../../templates/migration.rs.tera"),
        "pivot_model.rs.tera" => include_str!("../../templates/pivot_model.rs.tera"),
        "payloads.rs.tera" => include_str!("../../templates/payloads.rs.tera"),
        "test.rs.tera" => include_str!("../../templates/test.rs.tera"),
        "manager.rs.tera" => include_str!("../../templates/manager.rs.tera"),
        "relation.rs.tera" => include_str!("../../templates/relation.rs.tera"),
        _ => panic!("Unknown template: {}", template),
    };

    let rendered = tera.render_str(template_content, context).unwrap();
    let folder_path = if folder == "migration/src" {
        format!("./{}", folder)
    } else {
        format!("./src/{}", folder)
    };

    if !Path::new(&folder_path).exists() {
        create_dir_all(&folder_path).unwrap();
    }

    let folder_path = if folder == "migration/src" {
        format!(
            "{}/m{}_create_{}_table.rs",
            folder_path,
            chrono::Utc::now().format("%Y%m%d%H%M%S"),
            resource_name.to_lowercase()
        )
    } else {
        format!("{}/{}.rs", folder_path, file_name)
    };

    let mut file = File::create(&folder_path).unwrap();
    file.write_all(rendered.as_bytes()).unwrap();
    if folder != "migration/src" {
        append_to_mod_file(folder, &file_name, &format!("./src/{}/mod.rs", folder));
    }
}
