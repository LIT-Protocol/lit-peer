use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "config/proto/";
    let proto_file = "config/proto/chatter.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        // .type_attribute("NodeHeaderMetaData", "#[derive(serde::Deserialize, serde::Serialize)]")
        // .type_attribute("NodeRecordHeader", "#[derive(serde::Deserialize, serde::Serialize)]")
        // .type_attribute("NodeRecord", "#[derive(serde::Deserialize, serde::Serialize)]")
        // .type_attribute("NodeRecordFooter", "#[derive(serde::Deserialize, serde::Serialize)]")
        .file_descriptor_set_path(out_dir.join("chatter_descriptor.bin"))
        .compile_protos(&[proto_file], &[proto_dir])?;

    // INSERT GIT COMMIT HASH
    let output = Command::new("git").args(["rev-parse", "HEAD"]).output();

    let git_commit_hash = match output {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s.trim().to_string(),
            Err(e) => {
                eprintln!(
                    "Invalid UTF-8 output from git with error: {}.  No git commit hash will be inserted...",
                    e
                );
                "n/a".to_string()
            }
        },
        Err(e) => {
            eprintln!(
                "Failed to execute git command with error: {}.  No git commit hash will be inserted...",
                e
            );
            "n/a".to_string()
        }
    };

    let dest_path = Path::new("src/git_info.rs");
    let path_contents = format!(
        "pub const GIT_COMMIT_HASH: &str = \"{}\";\n",
        git_commit_hash
    );

    if let Err(e) = fs::write(dest_path, path_contents) {
        eprintln!(
            "Failed to write git_info.rs file with error: {}.  Exiting build.rs ...",
            e
        );
    }

    Ok(())
}
