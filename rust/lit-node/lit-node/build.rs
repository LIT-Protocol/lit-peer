//! Build script for `lit_node`.
//!
//! This file looks a bit more complex than a typical `tonic_build` setup because we also embed
//! the current git commit hash into the binary.
//!
//! Key design constraints:
//! - We **must not** write generated data into `src/` during builds (that dirties everyone's
//!   working tree and causes noisy `git status` output).
//! - Instead, we inject the hash as a **compile-time env var** via `cargo:rustc-env`, and
//!   `src/git_info.rs` reads it with `env!("GIT_COMMIT_HASH")`.
//! - Cargo does **not** rerun `build.rs` on every `cargo build`; it reruns only when inputs
//!   change. We therefore **watch git's `HEAD`** (and its referenced ref file) using
//!   `cargo:rerun-if-changed=...` so the embedded hash updates whenever the repo advances.
//! - For builds outside a git checkout (e.g. source tarballs), packagers can provide
//!   `GIT_COMMIT_HASH` (injected_hash) and we propagate it into the binary.
use std::process::Command;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // INSERT GIT COMMIT HASH
    println!("cargo:rerun-if-env-changed=GIT_COMMIT_HASH");
    let injected_hash = env::var("GIT_COMMIT_HASH")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let git_commit_hash: Option<String> = if let Some(injected_hash) = injected_hash {
        // When explicitly injected (e.g. source tarballs, CI), we don't need to query git or watch HEAD.
        Some(injected_hash)
    } else {
        // We ask git what the hash is and where to watch for changes.
        let (git_dir, git_head_hash): (Option<String>, Option<String>) = match Command::new("git")
            .args(["rev-parse", "--git-dir", "HEAD"])
            .output()
        {
            Ok(output) if output.status.success() => match String::from_utf8(output.stdout) {
                Ok(s) => {
                    let mut lines = s.lines().map(str::trim).filter(|l| !l.is_empty());
                    let git_dir = lines.next().map(|v| v.to_string());
                    let git_head_hash = lines.next().map(|v| v.to_string());
                    (git_dir, git_head_hash)
                }
                Err(e) => {
                    eprintln!(
                        "Invalid UTF-8 output from git with error: {}.  No git commit hash will be inserted...",
                        e
                    );
                    (None, None)
                }
            },
            Ok(_output) => (None, None), // not a git repo (or HEAD unavailable)
            Err(e) => {
                eprintln!(
                    "Failed to execute git command with error: {}.  No git commit hash will be inserted...",
                    e
                );
                (None, None)
            }
        };

        // Watching git's `HEAD` (and its referenced ref) makes the embedded hash update
        // whenever the repo advances.
        if let Some(git_dir) = git_dir.as_deref() {
            let head_path = format!("{git_dir}/HEAD");
            println!("cargo:rerun-if-changed={head_path}");

            // If HEAD points at a ref, also re-run when that ref file changes.
            if let Ok(head_contents) = fs::read_to_string(&head_path)
                && let Some(ref_path) = head_contents.trim().strip_prefix("ref: ")
            {
                let ref_file = format!("{git_dir}/{ref_path}");
                println!("cargo:rerun-if-changed={ref_file}");
            }
        }

        git_head_hash
    };

    // Make the head hash available at compile time (if known).
    if let Some(git_commit_hash) = git_commit_hash {
        println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_commit_hash);
    }

    Ok(())
}