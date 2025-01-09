# CRUD Generator

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Usage](#usage)
  - [1. JSON Configuration](#1-json-configuration)
  - [2. Run the Generator](#2-run-the-generator)
  - [3. Generated Files](#3-generated-files)
  - [4. Post-Generation Manual Steps](#4-post-generation-manual-steps)
  - [5. Customize Templates](#5-customize-templates)

---

## Overview

The **CRUD Generator** is a Rust project designed to simplify the creation of a fully functional CRUD (Create, Read, Update, Delete) system. It leverages **Tera** for templating and generates all the necessary files (models, handlers, managers, repositories, and migrations) based on a JSON configuration or command-line input.

This tool is particularly useful for rapidly scaffolding a Rust project with RESTful endpoints and database migrations, saving you time and effort during the initial stages of development.

---

## Features

- **Customizable CRUD generation**: Define your resource fields and relationships (One-to-One, One-to-Many, Many-to-Many) in a JSON file or interactively.
- **SeaORM integration**: Automatically generates models, repositories, and migrations compatible with SeaORM.
- **Flexible file templating**: Uses Tera templates to generate modular and reusable Rust files.
- **Relationship support**: Handles complex relationships and generates pivot tables for Many-to-Many relations.
- **Migration-ready**: Creates migration files for database schema changes.

---

## Usage

### 1. JSON Configuration

Create a JSON file (e.g., `resource.json`) defining your resource. Here's an example for a `user` resource:

```json
{
  "resource_name": "user", // ALWAYS LOWERCASE AND SNAKE_CASE
  "fields": [
    {
      "name": "uuid",
      "field_type": "Uuid",
      "primary_key": true,
      "unique": true,
      "nullable": false
    },
    {
      "name": "email",
      "field_type": "String",
      "primary_key": false,
      "unique": true,
      "nullable": false
    },
    {
      "name": "name",
      "field_type": "String",
      "primary_key": false,
      "unique": false,
      "nullable": false
    }
  ],
  "relations": [
    {
      "type": "one_to_many",
      "related_entity": "post",
      "from_column": "id",
      "to_column": "user_id"
    }
  ]
}
```

##### Field Types

Available field types:

- `String`: Text data
- `i32`: Integer numbers
- `f32`: Floating-point numbers
- `bool`: Boolean values
- `Uuid`: Unique identifiers
- `DateTime<Utc>`: Date and time values

##### Relation Types (WIP)

- `one_to_one`
- `one_to_many`
- `many_to_many`

### 2. Install cli

```bash
cargo install crud-generator
```

### 2. Run the Generator

```bash
crud-generator ./resource.json
```

You will be prompted to choose which methods (POST, GET, GET ALL, PUT, DELETE) you want to generate. (That's a multi-select)

### 3. Generated Files

The generator will output files in your src/ directory:

- `models/<resource>_model.rs`: Defines the SeaORM entity model.
- `handlers/<resource>_handler.rs`: RESTful endpoint handlers.
- `managers/<resource>_manager.rs`: Business logic layer.
- `repositories/<resource>_repository.rs`: Database repository for CRUD operations.
- `migration/src/m<timestamp>_create_<resource>_table.rs`: Database migration.

#### Generated API Endpoints

The generator creates the following REST endpoints:

- `POST /{{resource}}` - Create a new resource
- `GET /{{resource}}/{uuid}` - Get a specific resource by UUID
- `GET /{{resource}}` - List all resources
- `PUT /{{resource}}/{uuid}` - Update a specific resource
- `DELETE /{{resource}}/{uuid}` - Delete a specific resource

### 4. Post-Generation Manual Steps

This section outlines the additional manual tasks required to fully integrate the generated CRUD files into your project. While the CRUD Generator automates the creation of most components, some modifications—such as registering routes, linking migrations, or updating configuration files—need to be done manually to ensure everything works seamlessly.

Here are ALL the post-generation steps to fully integrate the generated CRUD :

#### 1. Modify the `src/app_config.rs` to add the new resource to the API router and register the necessary routes.

In order to do this, you'll need to add the following code to the end of file and replace the `API_RESOURCE` with the name of your resource.

```rust
#[derive(Default, Clone)]
pub struct Container {
    {...},
    pub {{API_RESOURCE}}_manager: Option<{{API_RESOURCE}}Manager>,
}

impl Container {
    pub fn new({...}, {{API_RESOURCE}}_repository: Arc<dyn {{API_RESOURCE}}Repository>) -> Container {
        Container {
            {...},
            {{API_RESOURCE}}_manager: Some({{API_RESOURCE}}Manager::new({{API_RESOURCE}}_repository)),
        }
    }
}
```

Next, you'll need to add the following code in the implementation of Router struct :

```rust
impl<'a> Router<'a> {
    {...}
    pub fn {{API_RESOURCE}}(&'a mut self) -> &mut Self {
        self.cfg.app_data(web::Data::new(
            self.container.{{API_RESOURCE}}_manager.as_ref().unwrap().clone(),
        ));
        register_{{API_RESOURCE}}_handlers(self.cfg);
        self
    }
}
```

Then, you must add the routing in the `configure_routes` function in the `src/app_config.rs` file :

```rust
pub fn configure_routes(cfg: &mut ServiceConfig, container: &Container) {
    let mut router = Router { cfg, container };
    router.api_doc().{...}.{{API_RESOURCE}}();
}
```

Finally, you'll need to add the repository in the `configure_app` function in the `src/app_config.rs` file :

```rust
pub fn configure_app() -> fn(cfg: &mut ServiceConfig, pool: SharedDbPool) {
    move |cfg: &mut ServiceConfig, pool: SharedDbPool| {
        {...}
        let {{API_RESOURCE}}_repository: Arc<dyn {{API_RESOURCE}}Repository> =
            Arc::new(Concrete{{API_RESOURCE}}Repository::new(pool.clone()));
        let container = Container::new({...}, {{API_RESOURCE}}_repository);
        configure_routes(cfg, &container);
    }
}
```

Don't forget to add necessary imports :

```rust
use crate::managers::{{API_RESOURCE}}_manager::{{API_RESOURCE}}Manager;
use crate::handlers::{{API_RESOURCE}}_handler::register_service as register_{{API_RESOURCE}}_handlers;
use crate::repositories::{{API_RESOURCE}}_repository::{{{API_RESOURCE}}Repository, Concrete{{API_RESOURCE}}Repository};
```

In order to get the Rapidoc documentation, you'll need to add the following code in the `src/handlers/apidoc_handler.rs` file :

```rust
use crate::handlers::{{API_RESOURCE}}_handler as {{API_RESOURCE}};

{...}

    nest(
        {...},
        (path="/{{API_RESOURCE}}", api={{API_RESOURCE}}::{{API_RESOURCE}}ApiDoc),
    )
```

You need to register the migration in the `migration/src/lib.rs` file :

```rust
{...}
mod {{MIGRATION_NAME}};
{...}
fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new({...}, {{MIGRATION_NAME}}::Migration)]
    }
```

And then run the new migration with the following command:

```shell
    cd migration
    cargo run
```

Congratulations! You have successfully added a new resource to your project. 🎉

### 5. Customize Templates

Templates are located in the `templates/` folder. Modify them to suit your project's requirements.
