//! CLI tool to generate OpenAPI specification files for the Lit Protocol Node API.
//!
//! Usage:
//!   cargo run -p openapi-gen
//!
//! This will generate:
//!   - openapi.json - OpenAPI 3.1 specification in JSON format

use openapi_gen::ApiDoc;
use std::fs;
use utoipa::OpenApi;

fn main() {
    let openapi = ApiDoc::openapi();

    match openapi.to_pretty_json() {
        Ok(json) => {
            if let Err(e) = fs::write("openapi.json", &json) {
                eprintln!("Failed to write openapi.json: {e}");
                std::process::exit(1);
            }
            println!("Generated openapi.json");
        }
        Err(e) => {
            eprintln!("Failed to generate JSON: {e}");
            std::process::exit(1);
        }
    }
}
