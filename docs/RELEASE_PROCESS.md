# Release Process

This document describes how to create a new release of Datadown.

## Overview

Releases are **tag-based** and fully automated via GitHub Actions. When you push a version tag, the CI system automatically:
1. Builds binaries for all major platforms (Windows, macOS, Linux)
2. Creates a GitHub Release with auto-generated changelog
3. Uploads all binaries and checksums
4. (Optional) Publishes to crates.io

## Package Structure

The project uses a **single unified crate** called `datadown` that provides both:
- **Library API**: Rust developers can add `datadown = "0.1.0"` to use the parsing and conversion functions
- **CLI Binary**: Users can install with `cargo install datadown` to get the command-line tool

This approach keeps the library and CLI in sync with a single version number and simplifies the release process.

## Release Workflow

### 1. Prepare the Release

**Update version numbers** in all relevant `Cargo.toml` files:

```bash
# Update version in these files to match (e.g., 0.2.0):
crates/datadown/Cargo.toml        # Main package (CLI + library)
bindings/python/Cargo.toml         # Python bindings
bindings/node/Cargo.toml           # Node.js bindings

# Also update if needed:
bindings/python/pyproject.toml (line 7: version = "0.2.0")
bindings/node/package.json (line 3: "version": "0.2.0")
```

**Commit the version changes:**

```bash
git add .
git commit -m "chore: bump version to 0.2.0"
git push origin main
```

### 2. Create and Push the Tag

**Create an annotated tag:**

```bash
# Format: v{MAJOR}.{MINOR}.{PATCH}
git tag -a v0.2.0 -m "Release v0.2.0

- Add feature X
- Fix bug Y
- Improve performance Z"

# Push the tag to trigger the release
git push origin v0.2.0
```

> **Note:** Use annotated tags (`-a`) to include a tag message. This message can be used in release notes.

### 3. Monitor the Release

1. Go to **Actions** tab in GitHub
2. Watch the "Release" workflow run
3. Verify all platform builds succeed
4. Check the **Releases** page for the new release

### 4. Verify the Release

After the workflow completes:

1. **Check binaries exist** for all platforms:
   - `datadown-{version}-x86_64-pc-windows-msvc.zip`
   - `datadown-{version}-x86_64-apple-darwin.tar.gz`
   - `datadown-{version}-aarch64-apple-darwin.tar.gz`
   - `datadown-{version}-x86_64-unknown-linux-gnu.tar.gz`
   - `datadown-{version}-x86_64-unknown-linux-musl.tar.gz`

2. **Download and test** one binary:
   ```bash
   # Example for macOS:
   wget https://github.com/USERNAME/datadown/releases/download/v0.2.0/datadown-0.2.0-x86_64-apple-darwin.tar.gz
   tar xzf datadown-0.2.0-x86_64-apple-darwin.tar.gz
   ./datadown-0.2.0-x86_64-apple-darwin/datadown --version
   ```

3. **Verify checksums**:
   ```bash
   # Download checksum file
   wget https://github.com/USERNAME/datadown/releases/download/v0.2.0/datadown-0.2.0-x86_64-apple-darwin.tar.gz.sha256

   # Verify (macOS/Linux)
   shasum -a 256 -c datadown-0.2.0-x86_64-apple-darwin.tar.gz.sha256
   ```

## Semantic Versioning

Follow [SemVer](https://semver.org/) guidelines:

- **MAJOR** (1.0.0): Breaking changes, incompatible API changes
- **MINOR** (0.1.0): New features, backwards-compatible
- **PATCH** (0.0.1): Bug fixes, backwards-compatible

### Pre-release Versions

For testing or beta releases, use pre-release tags:

```bash
git tag -a v0.2.0-beta.1 -m "Beta release"
git push origin v0.2.0-beta.1
```

Pre-releases are automatically marked as "Pre-release" on GitHub.

## Manual Release (Testing)

For testing the workflow without a tag:

1. Go to **Actions** → **Release** workflow
2. Click **Run workflow**
3. Enter version number (e.g., `0.2.0-test`)
4. Click **Run workflow**

This creates a release but requires manual tag creation.

## Troubleshooting

### Build Fails on Specific Platform

1. Check the Actions log for that platform
2. Common issues:
   - Missing Rust target: Add to workflow matrix
   - Cross-compilation issue: May need additional tools
   - Test failures: Ensure tests pass locally first

### Version Mismatch

If you pushed a tag but the version in `Cargo.toml` doesn't match:

1. Delete the tag locally and remotely:
   ```bash
   git tag -d v0.2.0
   git push --delete origin v0.2.0
   ```

2. Delete the GitHub Release (if created)

3. Fix the version in `Cargo.toml`

4. Create and push the tag again

### Crates.io Publishing Fails

The workflow includes optional crates.io publishing, but it requires:

1. **CARGO_TOKEN** secret in GitHub repository settings:
   - Go to [crates.io](https://crates.io/me) → Account Settings → API Tokens
   - Create new token
   - Add as `CARGO_TOKEN` in GitHub Settings → Secrets → Actions

2. Ensure you're a crate owner:
   ```bash
   cargo owner --add YOUR_USERNAME datadown
   ```

If publishing fails, releases still succeed (it's marked as `continue-on-error`).

> **Note:** The `datadown` crate includes both the library and CLI binary. Users can install it with `cargo install datadown` (CLI) or add it as a dependency in their `Cargo.toml` (library).

## Release Checklist

Before creating a release:

- [ ] All tests pass locally (`cargo test --workspace`)
- [ ] All changes are committed and pushed
- [ ] Version updated in `crates/datadown/Cargo.toml`
- [ ] Version numbers match in bindings (`bindings/python/Cargo.toml`, `bindings/node/Cargo.toml`)
- [ ] Python/Node package files updated (`pyproject.toml`, `package.json`)
- [ ] CHANGELOG or commit messages are clear
- [ ] Tag follows `v{MAJOR}.{MINOR}.{PATCH}` format
- [ ] Tag message describes key changes

After release:

- [ ] All platform binaries built successfully
- [ ] Downloaded and tested at least one binary
- [ ] Release notes look correct
- [ ] (Optional) Announced release on social media/forums
- [ ] (Optional) Update documentation website

## Platform Support Matrix

| Platform | Target Triple | Runs On | Notes |
|----------|---------------|---------|-------|
| Windows (64-bit) | x86_64-pc-windows-msvc | windows-latest | MSVC toolchain |
| macOS (Intel) | x86_64-apple-darwin | macos-13 | Intel Macs |
| macOS (Apple Silicon) | aarch64-apple-darwin | macos-latest | M1/M2/M3 Macs |
| Linux (GNU) | x86_64-unknown-linux-gnu | ubuntu-latest | glibc (most distros) |
| Linux (musl) | x86_64-unknown-linux-musl | ubuntu-latest | Static, Alpine Linux |

## Future Enhancements

Potential improvements to the release process:

- [ ] **Code signing**: Sign macOS and Windows binaries
- [ ] **Homebrew tap**: Auto-update formula on release
- [ ] **Chocolatey package**: Windows package manager
- [ ] **npm package**: Bundle binary for Node.js users
- [ ] **PyPI package**: Bundle binary for Python users
- [ ] **Docker images**: Multi-arch container images
- [ ] **Release notes automation**: Use conventional commits
- [ ] **Artifact attestation**: GitHub's supply chain security
