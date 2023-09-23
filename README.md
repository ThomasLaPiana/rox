# Rox

![crates.io](https://img.shields.io/crates/v/rox-cli.svg)
[![CI Checks](https://github.com/ThomasLaPiana/rox/actions/workflows/checks.yml/badge.svg)](https://github.com/ThomasLaPiana/rox/actions/workflows/checks.yml)

Composable build tool inspired by [Nox](https://nox.thea.codes/en/stable/), Make & [cargo-make](https://github.com/sagiegurari/cargo-make)

Rox gives you the ability to build your own devtools CLI using YAML files. Tasks and Pipelines are dynamically added to the CLI as subcommands at runtime. The flexibility of `rox` intends to makes it easier for dev teams to standardize their workflows without writing endless "glue" scripts.

The subcommands and their help messages are automatically populated at runtime from the `name` and `description` of each `task`.

## Table of Contents

- [Installation](#installation)
- [Roxfile Syntax](#roxfile-syntax)
  - [Version Requirements](#version-requirements)
  - [File Requirements](#file-requirements)
  - [Templates](#templates)
  - [Tasks](#tasks)
  - [Pipelines](#pipelines)
  - [Putting it all Together](#putting-it-all-together)
- [Usage Examples](#usage-examples)

## Installation

Rox can be installed via binaries provided with each release [here](https://github.com/ThomasLaPiana/rox/releases). As an alternative, it can also be installed via `cargo` with `cargo install rox-cli`.

## Roxfile Syntax

### Version Requirements

Version Requirements are used to ensure that any required CLI tool matches your specified version requirements.

```yaml
version_requirements:
  - command: "docker version --format {{.Client.Version}}" # Output: 20.10.23
    minimum_version: "20.10.7"
    maximum_version: "21.0.0"

  - command: "python --version" # Output: Python 3.9.13
    # Splits on spaces and grabs the last output token as the Version
    split: true 
    minimum_version: "3.8"
```

### File Requirements

File Requirements ensure that certain expected files are present.

```yaml
file_requirements:
  - path: "Cargo.toml" 
  
  - path: ".env"
    create_if_not_exists: true # Create the file if it doesn't exist, as opposed to throwing an error
```

### Templates

Templates allow you to specify templated commands that can be reused by `tasks`. Values are injected positionally. These are intended to facilitate code reuse and uniformity across similar but different commands.

```yaml
templates:
  - name: docker_build
    command: "docker build {path} -t rox:{image_tag}"
    symbols: ["{path}", "{image_tag}"]
```

### Tasks

Tasks are discrete units of execution. They're intended to be single shell commands that can then be composed via `pipelines`. They are also able to leverage `templates` by specifying one with `uses` and injecting values with `values`.

```yaml
tasks:
  - name: build-prod
    description: "Build the application dockerfile"
    uses: docker_build
    values: [".", "latest"]
    
  - name: "watch-run"
    description: "Run the application, restarting on file changes"
    command: "cargo watch -c -x run"
```

### Pipelines

Pipelines are the canonical way to chain together multiple tasks into a single unit of execution. They also support parallel execution with the `-p` flag but it is up to the user to ensure that the tasks can be safely executed in parallel.

```yaml
pipelines: 
  - name: example-pipeline
    description: Composes a few tasks
    tasks: ["task-a", "task-b", "task-c"]
```

### Putting it all together

Now that we've seen each individual piece of the Rox puzzle, we can put them all together into a full `roxfile`.

```yaml
version_requirements:
  - command: "docker version --format {{.Client.Version}}"
    minimum_version: "20.10.7"
    maximum_version: "21.0.0"

file_requirements:
  - path: ".env"
    create_if_not_exists: true

templates:
  - name: docker_build
    command: "docker build {path} -t rox:{image_tag}"
    symbols: ["{path}", "{image_tag}"]

pipelines:
  - name: build-all
    description: "Build a release artifact binary and Docker image"
    tasks: ["build-release-binary", "build-release-image"]

  - name: ci
    description: "Run all CI-related tasks"
    tasks: ["fmt", "test", "clippy-ci"]

tasks:

  - name: build-local
    description: "Build the application dockerfile"
    uses: docker_build
    values: [".", "local"]

  - name: build-prod
    description: "Build the application dockerfile"
    uses: docker_build
    values: [".", "latest"]

  - name: "clippy-ci"
    description: "Run Clippy with a non-zero exit if warnings are found."
    command: "cargo clippy -- -D warnings"

  - name: fmt
    command: "cargo fmt"

  - name: test
    command: "cargo test"
    description: "Run tests"

  # Release-related
  - name: build-release-binary
    description: "Build a release binary with cargo."
    command: "cargo build --release"

  - name: build-release-image
    description: "Build a production image for Docker."
    command: "docker build . -t rox:latest"

  - name: secret_task
    description: "This task isn't callable directly from the CLI, but is available to pipelines!"
    hide: true

```

## Usage Examples

The following are command-line examples for running `rox` with various flags and subcommands.

Show Tasks/Pipelines:

```sh
rox task
```

```sh
rox pl
```

<https://github.com/ThomasLaPiana/rox/assets/5105354/2041522d-4cb2-4c96-9655-c1802fdf16c8>

Run a Task:

```sh
rox task build-binary
```

<https://github.com/ThomasLaPiana/rox/assets/5105354/9f152b3b-8a65-4409-af5c-da029c3e8ae4>

Run a Pipeline:

```sh
rox pl ci
```

<https://github.com/ThomasLaPiana/rox/assets/5105354/02d99bc6-0dc1-4c33-a753-2868043c4d43>

Run a Pipeline in Parallel:

```sh
rox pl -p build-release-all
```

## Releasing

`Rox` is released by running `cargo release` locally.

Steps to Release:

1. Make sure that all desired changes are pushed up and merged to `main`
1. `cargo install cargo-release` (if not already installed)
1. `cargo release [major|minor|patch] --execute` - Updates the `Cargo.toml`, commits and pushes the change, and then publishes the crate to <crates.io>
1. `cargo release tag --execute` - Creates a git tag with the same version as the `Cargo.toml`
1. `cargo release push --execute` - Pushes the git tag
1. Finally, a CI job is automatically triggered to build and upload the release assets
