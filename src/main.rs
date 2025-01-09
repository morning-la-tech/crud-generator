use inflector::Inflector;
use inquire::MultiSelect;
use std::env;
use std::fs::File;
use std::process::Command;
use tera::Tera;

mod generators;
mod models;
mod utils;

use generators::{generate_files, generate_migration, generate_pivot_file};
use models::Resource;
use utils::{file::check_if_files_exist, template::*};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "\nError: Missing resource JSON file path\nUsage: cargo run -- <path_to_resource_json>"
        );
        std::process::exit(1);
    }

    // We define options
    let methods = vec!["POST", "GET", "GET ALL", "PUT", "DELETE"];

    // Ask for multiple selections
    let selected_methods = MultiSelect::new(
        "Choose methods to generate :\n/!\\ Careful: if you don't generate ALL methods, tests won't be generated either.",
        methods,
    )
    .prompt()
    .unwrap_or_else(|_| vec![]); // Default value in case of cancellation

    // Display the result
    if selected_methods.is_empty() {
        eprintln!("You didn't select any method.");
        std::process::exit(0);
    }

    let resource_file_path = &args[1];
    eprintln!("\nUsing resource JSON file: {}", resource_file_path);
    let input: Resource = match serde_json::from_reader(File::open(resource_file_path).unwrap()) {
        Ok(resource) => resource,
        Err(e) => {
            eprintln!(
                "\nError: Failed to parse JSON file '{}'\nDetails: {}",
                resource_file_path, e
            );
            std::process::exit(1);
        }
    };

    if let Err(error_message) = check_if_files_exist(&input.resource_name.to_snake_case()) {
        eprintln!("{}", error_message);
        std::process::exit(1);
    }

    eprintln!(
        "\nGenerating CRUD files for resource: {}",
        input.resource_name
    );

    let mut tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "\nError: Failed to initialize template engine\nDetails: {}",
                e
            );
            std::process::exit(1);
        }
    };

    tera.register_filter("snake_case", snake_case_filter);
    tera.register_filter("pascal_case", pascal_case_filter);
    tera.register_filter("plural", plural_filter);
    tera.register_filter("snake_case_to_kebab_case", snake_case_to_kebab_case_filter);

    generate_migration(&mut tera, &input);
    generate_files(&mut tera, &input, selected_methods);

    if let Some(relations) = &input.relations {
        for relation in relations {
            if relation.r#type == "many_to_many" {
                generate_pivot_file(&mut tera, &input, relation);
            }
        }
    }

    format_generated_code();
}

fn format_generated_code() {
    eprintln!("\nFormatting generated files...");

    let format_result = Command::new("cargo").arg("fmt").current_dir(".").status();

    match format_result {
        Ok(status) => {
            if status.success() {
                eprintln!("Code formatting completed successfully!");
            } else {
                eprintln!("Warning: Code formatting failed with status: {}", status);
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to run cargo fmt: {}", e);
        }
    }

    let migration_format_result = Command::new("cargo")
        .arg("fmt")
        .current_dir("./migration")
        .status();

    match migration_format_result {
        Ok(status) => {
            if status.success() {
                eprintln!("Migration code formatting completed successfully!");
            } else {
                eprintln!(
                    "Warning: Migration code formatting failed with status: {}",
                    status
                );
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to run cargo fmt in migration: {}", e);
        }
    }
}
