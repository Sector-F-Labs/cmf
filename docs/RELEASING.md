# Release Guide

This document outlines the process for releasing a new version of CMF to crates.io and GitHub.

## Prerequisites

- Commit rights to the repository
- crates.io API token configured locally (`~/.cargo/credentials.toml`)
- GitHub CLI (`gh`) installed and authenticated

## Release Process

### 1. Determine the Version Bump

Check the git history since the last release to determine the appropriate version bump:

```bash
git log <last-release-tag>..HEAD --oneline
```

Use [semantic versioning](https://semver.org/):
- **MAJOR** (x.0.0): Breaking changes
- **MINOR** (0.x.0): New features (backwards compatible)
- **PATCH** (0.0.x): Bug fixes (backwards compatible)

### 2. Write Release Notes

Create a summary of changes since the last release. Include:
- New features
- Bug fixes
- Breaking changes (if any)
- Dependency updates
- Contributors

Use the commit history as reference:

```bash
git log <last-release-tag>..HEAD --pretty=format:"%h - %s" | sort
```

### 3. Update Version in Cargo.toml

Update the `version` field in `Cargo.toml` to the new version number:

```toml
[package]
version = "0.x.x"
```

### 4. Commit and Push

Commit the version bump:

```bash
git add Cargo.toml Cargo.lock
git commit -m "Bump version to 0.x.x"
git push origin master
```

### 5. Create GitHub Release

Create a GitHub release with the release notes:

```bash
gh release create v0.x.x --title "v0.x.x" --notes "$(cat <<'EOF'
## Changes

- Feature/fix 1
- Feature/fix 2
- ...

EOF
)"
```

Alternatively, create the release via the GitHub web UI at https://github.com/sector-f-labs/cmf/releases/new

### 6. Publish to crates.io

Ensure the working directory is clean, then publish:

```bash
cargo publish
```

The command will:
- Package the crate
- Verify the build
- Upload to crates.io
- Wait for the crate to be available

If there are uncommitted changes in `Cargo.lock`, commit them before publishing.

## Verification

After release, verify the new version is available:

- **crates.io**: https://crates.io/crates/cmf
- **GitHub Releases**: https://github.com/sector-f-labs/cmf/releases
- **Git tags**: `git tag -l` should show the new version

## Troubleshooting

### Publish fails with dirty working directory

Ensure all changes are committed:

```bash
git status
git add .
git commit -m "Update lock files"
cargo publish
```

### Release already exists on crates.io

A version can only be published once. If you need to re-release, use a new version number.

### Need to yank a release

If there's a critical issue:

```bash
cargo yank --vers 0.x.x
```

This prevents new projects from depending on that version.
