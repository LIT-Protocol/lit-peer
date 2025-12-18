//! Manages the process of publishing release metadata to GitHub.
//!
//! This module contains the logic for gathering release information, committing it
//! to a git repository, and creating a formal GitHub Release. This entire process
//! is considered a non-critical "side-effect" of a successful build and release.
//! A failure here will be logged but will not fail the overall release workflow.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio as ProcessStdio;

use chrono::Utc;
use octocrab::Octocrab;
use tokio::process::Command as TokioCommand;

use lit_core::config::LitConfig;
use lit_os_core::error::{generic_err, validation_err};
use lit_os_prov_core::release::create::types::CreateRelease;

use crate::cmd::os::guest::template::release::GuestTemplateRelease;
use crate::cmd::os::guest::template::release_info::{
    ArtifactsInfo, GitInfo, ReleaseInfo, SourceInfo, VerificationInfo,
};
use crate::error::Result;
use crate::guest::template::GuestTemplateItem;

/// Schema version for release metadata JSON
const SCHEMA_VERSION: &str = "1.0.0";

/// Base path where lit-os and lit-assets repos are mounted (CI environment)
const ASSETS_BASE_PATH: &str = "/opt/assets";

// --- Main Public Function ---

/// Primary entry point for generating and publishing release information.
///
/// This function orchestrates the gathering of metadata, committing it to the
/// data repository, and creating a public GitHub release.
pub(crate) async fn generate_and_publish_release_info(
    _config: &LitConfig, args: &GuestTemplateRelease, item: &GuestTemplateItem,
    release: &CreateRelease, manifest_cid: &str,
) -> Result<()> {
    println!("📦 Generating and publishing release information to GitHub");

    let release_info = gather_metadata(args, item, release, manifest_cid).await?;

    // GitPublisher ensures cleanup via Drop trait, even on error
    let publisher = GitPublisher::new(ASSETS_BASE_PATH, &args.data_branch).await?;
    publisher.commit_and_push_release_data(&release_info).await?;

    create_github_release(args, &release_info).await?;

    println!("✅ Successfully published release information to GitHub");
    Ok(())
}

// --- Metadata Gathering ---

/// Collects all release metadata from git, the build environment, and release artifacts.
async fn gather_metadata(
    args: &GuestTemplateRelease, item: &GuestTemplateItem, release: &CreateRelease,
    manifest_cid: &str,
) -> Result<ReleaseInfo> {
    println!("🔍 Gathering release metadata...");

    let base_repo_path = PathBuf::from(ASSETS_BASE_PATH);
    let lit_assets_path = base_repo_path.join("lit-assets");
    let lit_os_path = base_repo_path.join("lit-os");

    // Validate environment before proceeding
    if !lit_assets_path.exists() {
        return Err(validation_err(
            format!("lit-assets not found at {:?}", lit_assets_path),
            Some(format!(
                "Expected directory structure: {}/{{lit-assets,lit-os}}",
                ASSETS_BASE_PATH
            )),
        ));
    }
    if !lit_os_path.exists() {
        return Err(validation_err(
            format!("lit-os not found at {:?}", lit_os_path),
            Some(format!(
                "Expected directory structure: {}/{{lit-assets,lit-os}}",
                ASSETS_BASE_PATH
            )),
        ));
    }

    // The canonical version is sourced from the lit-node's Cargo.toml.
    let version = get_lit_node_version(&lit_assets_path).unwrap_or_else(|e| {
        eprintln!(
            "Warning: could not determine lit-node semantic version: {}. Falling back to build_id.",
            e
        );
        item.build_env.build_id.as_ref().cloned().unwrap_or_else(|| "unknown".to_string())
    });

    let lit_os_git_info = get_git_info(&lit_os_path).await?;
    let lit_assets_git_info = get_git_info(&lit_assets_path).await?;

    let gpg_key_fingerprint = run_git_command(&lit_assets_path, &["config", "user.signingkey"])
        .await
        .unwrap_or_else(|_| "GPG_KEY_NOT_CONFIGURED".to_string());

    println!(
        "  - Git info for lit-os: branch '{}', commit '{}'",
        lit_os_git_info.branch, lit_os_git_info.commit
    );
    println!(
        "  - Git info for lit-assets: branch '{}', commit '{}'",
        lit_assets_git_info.branch, lit_assets_git_info.commit
    );
    println!("  - GPG signing key: {}", gpg_key_fingerprint);

    let docker_image = item
        .build_env
        .build_env_img
        .as_ref()
        .map(|repo| format!("{}:{}", repo, release.release_id()))
        .unwrap_or_else(|| "No docker image for this release".to_string());

    // Network name must be explicitly provided
    let network_name = args.network_name.as_ref().map(|n| n.to_string()).ok_or_else(|| {
        validation_err(
            "Network name is required for release publication",
            Some(
                "Use --network-name flag or ensure calling code provides a valid network name"
                    .to_string(),
            ),
        )
    })?;

    let signed_git_tag = format!("v{}-{}", &version, &network_name);

    Ok(ReleaseInfo {
        schema_version: SCHEMA_VERSION.to_string(),
        release_id: release.release_id().to_string(),
        version,
        network_name,
        release_date: Utc::now(),
        status: "active".to_string(),
        source: SourceInfo { lit_os: lit_os_git_info, lit_assets: lit_assets_git_info },
        artifacts: ArtifactsInfo { manifest_cid: manifest_cid.to_string(), docker_image },
        verification: VerificationInfo { signed_git_tag, gpg_key_fingerprint },
    })
}

// --- Git Publishing Logic ---

/// A wrapper for performing git operations in a repository, ensuring cleanup.
struct GitPublisher {
    repo_path: PathBuf,
    data_branch: String,
    original_branch: String,
}

impl GitPublisher {
    /// Creates a new publisher, stashing the original branch name for later restoration.
    async fn new(base_path: &str, data_branch: &str) -> Result<Self> {
        let repo_path = PathBuf::from(base_path).join("lit-assets");
        if !repo_path.exists() {
            return Err(validation_err(format!("Repository not found at {:?}", repo_path), None));
        }

        let original_branch =
            run_git_command(&repo_path, &["rev-parse", "--abbrev-ref", "HEAD"]).await?;
        println!("  - Stashed current branch name '{}'.", original_branch);

        Ok(Self { repo_path, data_branch: data_branch.to_string(), original_branch })
    }

    /// Performs the core git operations to commit and push release metadata.
    async fn commit_and_push_release_data(&self, release_info: &ReleaseInfo) -> Result<()> {
        println!("📦 Committing and pushing release data to the '{}' branch...", &self.data_branch);

        // Switch to the data branch and pull the latest changes to avoid conflicts.
        run_git_command(&self.repo_path, &["checkout", &self.data_branch]).await?;
        run_git_command(&self.repo_path, &["pull", "--rebase", "origin", &self.data_branch])
            .await?;
        println!("  - Switched to branch '{}' and pulled latest.", &self.data_branch);

        let release_json_path = self
            .repo_path
            .join("release-data-store")
            .join(&release_info.network_name)
            .join(format!("{}.json", release_info.release_id));

        let parent_dir = release_json_path.parent().ok_or_else(|| {
            generic_err("Could not determine parent directory for release file", None)
        })?;
        fs::create_dir_all(parent_dir)
            .map_err(|e| generic_err(e, Some("Failed to create network directory".into())))?;

        let json_content = serde_json::to_string_pretty(release_info)
            .map_err(|e| generic_err(e, Some("Failed to serialize release info".into())))?;
        fs::write(&release_json_path, json_content)
            .map_err(|e| generic_err(e, Some("Failed to write release.json".into())))?;
        println!("  - Created release file at: {:?}", release_json_path);

        // Sign the metadata file using the GPG key configured in git.
        let gpg_key = self.get_gpg_key().await?;
        self.sign_file(&release_json_path, &gpg_key).await?;
        println!("  - Successfully signed release.json with key {}", gpg_key);

        // Add, commit (with GPG signature), and push the changes.
        run_git_command(&self.repo_path, &["add", "release-data-store"]).await?;
        let commit_message =
            format!("chore(release): Add release data for {}", release_info.release_id);
        run_git_command(
            &self.repo_path,
            &["commit", &format!("-S{}", gpg_key), "-m", &commit_message],
        )
        .await?;
        run_git_command(&self.repo_path, &["push", "origin", &self.data_branch]).await?;
        println!("  - Successfully committed and pushed release data.");

        Ok(())
    }

    async fn get_gpg_key(&self) -> Result<String> {
        match run_git_command(&self.repo_path, &["config", "user.signingkey"]).await {
            Ok(key) if !key.is_empty() => Ok(key),
            _ => Err(validation_err(
                "GPG signing key is not configured for Git.",
                Some("Please run 'git config --global user.signingkey YOUR_GPG_KEY_ID' and ensure the gpg-agent is running.".into()),
            )),
        }
    }

    async fn sign_file(&self, file_path: &Path, gpg_key: &str) -> Result<()> {
        let parent_dir = file_path
            .parent()
            .ok_or_else(|| generic_err("Cannot sign file in root directory", None))?;
        let file_str = file_path
            .to_str()
            .ok_or_else(|| generic_err("File path contains invalid UTF-8", None))?;

        let gpg_output = TokioCommand::new("gpg")
            .current_dir(parent_dir)
            .args(["--detach-sign", "--armor", "-u", gpg_key, file_str])
            .output()
            .await
            .map_err(|e| generic_err(e, Some("Failed to execute gpg signing command".into())))?;

        if !gpg_output.status.success() {
            return Err(generic_err(
                format!("GPG signing failed: {}", String::from_utf8_lossy(&gpg_output.stderr)),
                None,
            ));
        }
        Ok(())
    }
}

/// The Drop implementation ensures that we always attempt to return to the original git branch,
/// even if an error occurs during the publishing process.
impl Drop for GitPublisher {
    fn drop(&mut self) {
        println!("  - Returning to original branch '{}'.", self.original_branch);
        let repo_path_str = self.repo_path.to_str().unwrap_or(".");

        // We use a blocking `std::process::Command` here because `drop` cannot be async.
        let status = std::process::Command::new("git")
            .current_dir(&self.repo_path)
            .args(["checkout", &self.original_branch])
            .status();

        if let Err(e) = status {
            eprintln!(
                "FATAL: Failed to execute checkout command to restore original branch '{}' in '{}'. Your repository may be in a bad state. Error: {}",
                self.original_branch, repo_path_str, e
            );
        } else if !status.as_ref().unwrap().success() {
            eprintln!(
                "FATAL: Failed to checkout original branch '{}' in '{}'. Your repository may be in a bad state. Please fix it manually.",
                self.original_branch, repo_path_str
            );
        }
    }
}

// --- GitHub Release Logic ---

/// Creates a GitHub release with embedded JSON metadata and an auto-generated tag.
async fn create_github_release(
    args: &GuestTemplateRelease, release_info: &ReleaseInfo,
) -> Result<()> {
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| validation_err("GITHUB_TOKEN environment variable not set", None))?;

    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .map_err(|e| generic_err(e, Some("Failed to create GitHub client".into())))?;

    let repo_parts: Vec<&str> = args.github_repo.split('/').collect();
    if repo_parts.len() != 2 {
        return Err(validation_err("Invalid GitHub repository format. Expected owner/repo", None));
    }
    let (owner, repo) = (repo_parts[0], repo_parts[1]);

    let release_tag = &release_info.verification.signed_git_tag;
    let release_title = format!("v{} - {}", release_info.version, release_info.network_name);

    let json_body = serde_json::to_string_pretty(release_info)
        .map_err(|e| generic_err(e, Some("Failed to serialize release info to JSON".into())))?;

    let release_body = format!(
        "## Release Notes for {} - {}\n\n**Network:** {}\n\n### Release Metadata\n\n```json\n{}\n```",
        &release_title,
        release_info.release_date.format("%B %d, %Y"),
        release_info.network_name,
        json_body
    );

    let repo_path = PathBuf::from(ASSETS_BASE_PATH).join("lit-assets");
    let commit_hash = run_git_command(&repo_path, &["rev-parse", "HEAD"]).await?;

    println!("  - Creating GitHub release with tag: {}", &release_tag);
    println!("  - Pointing tag to commit: {}", &commit_hash);

    octocrab
        .repos(owner, repo)
        .releases()
        .create(release_tag)
        .target_commitish(&commit_hash)
        .name(&release_title)
        .body(&release_body)
        .send()
        .await
        .map_err(|e| generic_err(e, Some("Failed to create GitHub release".into())))?;

    Ok(())
}

// --- Utility Functions ---

/// Retrieves the current branch and commit hash from a git repository.
async fn get_git_info(repo_path: &Path) -> Result<GitInfo> {
    let branch = run_git_command(repo_path, &["rev-parse", "--abbrev-ref", "HEAD"]).await?;
    let commit = run_git_command(repo_path, &["rev-parse", "HEAD"]).await?;
    Ok(GitInfo { branch, commit })
}

/// Reads the semantic version from the lit-node Cargo.toml file using a TOML parser.
fn get_lit_node_version(lit_assets_path: &Path) -> Result<String> {
    let cargo_toml_path = lit_assets_path.join("rust/lit-node/lit-node/Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path).map_err(|e| {
        generic_err(
            format!("Failed to read lit-node Cargo.toml at {:?}: {}", cargo_toml_path, e),
            None,
        )
    })?;

    let toml_value: toml::Value = content
        .parse()
        .map_err(|e| generic_err(format!("Failed to parse Cargo.toml: {}", e), None))?;

    toml_value
        .get("package")
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            generic_err(
                format!("Could not find version in [package] section of {:?}", cargo_toml_path),
                None,
            )
        })
}

/// Executes a git command in the specified repository and returns stdout.
async fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = TokioCommand::new("git")
        .current_dir(repo_path)
        .args(args)
        .stdout(ProcessStdio::piped())
        .stderr(ProcessStdio::piped())
        .output()
        .await
        .map_err(|e| {
            generic_err(e, Some(format!("Failed to execute git command in {:?}", repo_path)))
        })?;

    if !output.status.success() {
        return Err(generic_err(
            format!(
                "Git command `git {}` failed with exit code {:?}: {}\n---\nSTDERR:{}",
                args.join(" "),
                output.status.code(),
                String::from_utf8_lossy(&output.stderr).trim(),
                String::from_utf8_lossy(&output.stderr).trim()
            ),
            None,
        ));
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .map_err(|e| generic_err(e, Some("Git command output was not valid UTF-8".into())))
}
