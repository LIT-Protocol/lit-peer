# Lit CLI - OS Commands

Command-line interface for managing Lit Protocol OS guest instances and templates.

## Features

- **Guest Instance Management** - Create, start, stop, and monitor guest VMs
- **Template Management** - Build and release guest templates to IPFS
- **Release Automation** - Publish releases with automated metadata generation and GitHub integration
- **IPFS Integration** - Upload and pin artifacts to IPFS with remote pinning support
- **Docker Image Publishing** - Automatically tag and push build environment images

---

## Release Command

The `lit os guest template release` command handles the complete release workflow:

### Basic Usage

```bash
lit os guest template release <BUILD_ID> \
  --subnet-id <SUBNET_ID> \
  [--network-name <NETWORK>] \
  [--github-repo <OWNER/REPO>] \
  [--data-branch <BRANCH>]
```

### Examples

```bash
# Production release with explicit network
lit os guest template release c6d133d4 \
  --subnet-id 4279a2CBc04b1DA986041DD4cF25D428510e0f32 \
  --network-name naga-test

# Development release (uses build environment as network)
lit os guest template release abc12345 \
  --subnet-id 1234567890abcdef1234567890abcdef12345678
```

### Valid Network Names

- **Production:** `datil-prod`, `naga-prod`
- **Staging:** `naga-staging`
- **Test:** `datil-test`, `naga-test`
- **Development:** `datil-dev`, `naga-dev`, `internal-dev`
- **Proto:** `datil-proto`

---

## Prerequisites for Releases

### 1. GPG Key Setup (Required)

The release command requires a GPG key for signing commits and release metadata.

#### Generate GPG Key

```bash
# Generate a new GPG key
gpg --full-generate-key
# Choose: RSA and RSA, 4096 bits, no expiration

# List your keys and copy the key ID
gpg --list-secret-keys --keyid-format=long
# Output: sec   rsa4096/68E00C6FFF28AFF0 2025-10-12 [SC]
#                       ^^^^^^^^^^^^^^^^ This is your key ID
```

#### Configure Git

```bash
# Set your GPG key for Git
git config --global user.signingkey 68E00C6FFF28AFF0
git config --global commit.gpgsign true

# Test that signing works
echo "test" | gpg --clearsign
```

#### Add GPG Key to GitHub

```bash
# Export your public key
gpg --armor --export 68E00C6FFF28AFF0 > my-gpg-key.asc

# Go to: GitHub Settings → SSH and GPG keys → New GPG key
# Paste the contents of my-gpg-key.asc
```

**Why GPG is required:**
- Ensures release metadata integrity
- Allows public verification of who authorized each release
- Creates a cryptographic chain of trust from CLI → Git → GitHub

**Error if not configured:**
```
Error: GPG signing key is not configured for Git.
Please run 'git config --global user.signingkey YOUR_GPG_KEY_ID'
and ensure the gpg-agent is running.
```

---

### 2. GitHub Token Setup (Required)

The release command needs a GitHub Personal Access Token (PAT) to create releases.

#### Create Personal Access Token

1. Go to: **GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)**
2. Click: **"Generate new token (classic)"**
3. Select required scopes:
   - ✅ `repo` (Full control of private repositories)
   - ✅ `workflow` (Update GitHub Action workflows)
4. Generate and copy the token

#### Configure Environment Variable

```bash
# Set the token as an environment variable
export GITHUB_TOKEN="ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"

# Make it persistent (add to ~/.bashrc or ~/.zshrc)
echo 'export GITHUB_TOKEN="ghp_your_token_here"' >> ~/.bashrc
source ~/.bashrc

# Verify it works
curl -H "Authorization: token $GITHUB_TOKEN" https://api.github.com/user
```

**Security note:** Keep your token secure! Never commit it to Git or share it publicly.

---

### 3. Path Requirements (CI Environment)

The release command expects repositories to be mounted at:

```
/opt/assets/
├── lit-assets/   # Must contain: rust/lit-node/lit-node/Cargo.toml
└── lit-os/       # Must contain: .git/
```

**Why?**
- CLI reads semantic version from `lit-node/Cargo.toml`
- CLI fetches git branch/commit info from both repos
- Standard path ensures consistency across CI environments

---

## What the Release Command Does

When you run `lit os guest template release`, the following happens automatically:

### 1. IPFS Release (Primary)
- ✅ Builds and uploads artifacts to IPFS
- ✅ Generates release manifest with measurements
- ✅ Pins artifacts to remote IPFS (Pinata)
- ✅ Submits release to provisioning API

### 2. Docker Image Publishing (If applicable)
- ✅ Tags Docker image with release ID
- ✅ Pushes to Docker Hub

### 3. GitHub Integration
- ✅ Gathers metadata (git commits, version, GPG key)
- ✅ Commits `release-data-store/<network>/<releaseId>.json` to `releases-info` branch
- ✅ GPG signs the commit
- ✅ Creates GitHub Release with tag `v<version>-<network>`
- ✅ Embeds JSON metadata in release body
- ✅ Triggers GitHub Actions workflow to deploy public website

### Release Metadata Schema

Each release generates a JSON file with the following structure:

```json
{
  "schemaVersion": "1.0.0",
  "releaseId": "c6d133d44279a2cbc04b1da...",
  "version": "0.3.52",
  "networkName": "datil-test",
  "releaseDate": "2025-10-21T14:30:00Z",
  "status": "active",
  "source": {
    "litOs": { "branch": "main", "commit": "8af61b1..." },
    "litAssets": { "branch": "develop", "commit": "6cd2d64..." }
  },
  "artifacts": {
    "manifestCid": "QmXdXc3LBc52d44kVKSiSTgd...",
    "dockerImage": "litprotocol/build-env:c6d133d4..."
  },
  "verification": {
    "signedGitTag": "v0.3.52-datil-test",
    "gpgKeyFingerprint": "68E00C6FFF28AFF0"
  }
}
```

---

## Error Handling

### Non-Blocking GitHub Publishing

If GitHub publishing fails, the CLI logs a warning but **does not fail the release**:

```
Warning: Failed to publish release information to GitHub: <error details>
```

**What this means:**
- The IPFS release and provisioning API submission always proceed
- The release is still valid and functional
- Operators can manually retry GitHub publishing later if needed

### Common Errors

#### GPG Key Not Configured
```
Error: GPG signing key is not configured for Git.
```
**Solution:** Follow the [GPG Key Setup](#1-gpg-key-setup-required) instructions above.

#### GitHub Token Not Set
```
Error: GITHUB_TOKEN environment variable not set
```
**Solution:** Follow the [GitHub Token Setup](#2-github-token-setup-required) instructions above.

#### Invalid Network Name
```
error: invalid value 'datil-poduction' for '--network-name <NETWORK>'
  [possible values: datil-prod, datil-test, datil-dev, ...]
```
**Solution:** Use one of the [valid network names](#valid-network-names).

#### Repository Path Not Found
```
Error: lit-assets not found at "/opt/assets/lit-assets"
Expected directory structure: /opt/assets/{lit-assets,lit-os}
```
**Solution:** Ensure repositories are mounted at the [expected paths](#3-path-requirements-ci-environment).

---

## Public Release Info Store

All releases are automatically published to a public website:

**URL:** https://lit-protocol.github.io/lit-peer/

**Features:**
- Search and filter releases by network, version, or release ID
- View complete release metadata including git commits and IPFS CIDs
- Direct links to GitHub commits, IPFS explorer, and Docker Hub
- Historical record of all releases

**How it works:**
1. CLI creates GitHub Release with tag `v<version>-<network>`
2. GitHub triggers "Deploy Release Info Store UI" workflow
3. Workflow builds and deploys static site to GitHub Pages (~2-3 min)

---

## Development

### Building

```bash
cd rust/lit-os/lit-cli-os
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name
```

---
