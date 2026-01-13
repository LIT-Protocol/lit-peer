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
    insert_git_commit_hash().expect("Failed to insert git commit hash.");
    Ok(())
}

fn insert_git_commit_hash() -> Result<(), Box<dyn std::error::Error>> {
    let src_hash = dirhash_fast::file::dir::dirhash::hash_directory("src".as_ref());
    println!("Source directory hash: {}", src_hash);
    let git_info_path = Path::new("src/git_info.rs");
    if git_info_path.exists() {
        let git_info_contents = fs::read_to_string(git_info_path).unwrap();
        if git_info_contents.contains(&src_hash) {
            println!("Source directory hash is already in git_info.rs file.  No need to update.");
            return Ok(());
        }
    }

    let output = Command::new("git").args(["rev-parse", "HEAD"]).output();

    let git_commit_hash = match output {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s.trim().to_string(),
            Err(e) => {
                eprintln!(
                    "Invalid UTF-8 output from git with error: {e}.  No git commit hash will be inserted...",
                );
                "n/a".to_string()
            }
        },
        Err(e) => {
            eprintln!(
                "Failed to execute git command with error: {e}.  No git commit hash will be inserted...",
            );
            "n/a".to_string()
        }
    };

    if !git_commit_hash.is_empty() {
        let path_contents = format!("pub const GIT_COMMIT_HASH: &str = \"{git_commit_hash}\";\n",);

        if let Err(e) = fs::write(git_info_path, path_contents) {
            eprintln!("Failed to write git_info.rs file with error: {e}.  Exiting build.rs ...",);
        }
    }
    Ok(())
}
