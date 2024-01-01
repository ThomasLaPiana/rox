# Releasing

`Rox` is released by running `cargo release` locally.

Steps to Release:

1. Make sure that all desired changes are pushed up and merged to `main`
1. `cargo install cargo-release` (if not already installed)
1. `cargo release [major|minor|patch] --execute` - Updates the `Cargo.toml`, commits and pushes the change, and then publishes the crate to <crates.io>
1. `cargo release tag --execute` - Creates a git tag with the same version as the `Cargo.toml`
1. `cargo release push --execute` - Pushes the git tag
1. Finally, a CI job is automatically triggered to build and upload the release assets
