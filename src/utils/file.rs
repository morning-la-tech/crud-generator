use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn check_if_files_exist(resource_name: &str) -> Result<(), String> {
    let base_path = "./src";
    let files_to_check = vec![
        (
            "model",
            format!("{}/models/{}.rs", base_path, resource_name),
        ),
        (
            "handler",
            format!("{}/handlers/{}_handler.rs", base_path, resource_name),
        ),
        (
            "repository",
            format!("{}/repositories/{}_repository.rs", base_path, resource_name),
        ),
        (
            "manager",
            format!("{}/managers/{}_manager.rs", base_path, resource_name),
        ),
        (
            "payload",
            format!("{}/payloads/{}_payloads.rs", base_path, resource_name),
        ),
    ];

    let mut existing_files = Vec::new();
    for (file_type, file_path) in files_to_check {
        if Path::new(&file_path).exists() {
            existing_files.push(format!("- {} file at: {}", file_type, file_path));
        }
    }

    if !existing_files.is_empty() {
        let error_message = format!(
            "\nError: Cannot generate CRUD for resource '{}' because the following files already exist:\n{}\n\nPlease remove these files first if you want to regenerate them.",
            resource_name,
            existing_files.join("\n")
        );
        return Err(error_message);
    }

    Ok(())
}

pub fn append_to_mod_file(_folder: &str, resource_name: &str, mod_file_path: &str) {
    if Path::new(&mod_file_path).exists() {
        let mut mod_file = OpenOptions::new()
            .append(true)
            .open(&mod_file_path)
            .unwrap();
        writeln!(mod_file, "#[cfg(test)]").unwrap();
        writeln!(mod_file, "mod {};", resource_name).unwrap();
    } else {
        let mut mod_file = BufWriter::new(File::create(&mod_file_path).unwrap());
        writeln!(mod_file, "#[cfg(test)]").unwrap();
        writeln!(mod_file, "mod {};", resource_name).unwrap();
    }
}
