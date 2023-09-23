# Rox

![crates.io](https://img.shields.io/crates/v/rox-cli.svg)
[![CI Checks](https://github.com/ThomasLaPiana/rox/actions/workflows/checks.yml/badge.svg)](https://github.com/ThomasLaPiana/rox/actions/workflows/checks.yml)

Composable build tool inspired by [Nox](https://nox.thea.codes/en/stable/), Make & [cargo-make](https://github.com/sagiegurari/cargo-make)

Rox gives you the ability to build your own devtools CLI using YAML files. Tasks and Pipelines are dynamically added to the CLI at runtime. It has fast startup times and full cross-platform compabitility. Being both performant and flexible makes it easier for dev teams to standardize their workflows without writing endless "glue" scripts.

The subcommands and their help messages are automatically populated at runtime from the `name` and `description` of each `target`.

See the [roxfile.yml](roxfile.yml) for an idea of the planned end-state of syntax and functionality! This is also the Roxfile used for this repo.

## Table of Contents

- [Installation](#installation)
- [Roxfile Syntax](#roxfile-syntax)
  - [Version Requirements](#version-requirements)
  - [File Requirements](#file-requirements)
  - [Templates](#templates)
  - [Tasks](#tasks)
  - [Pipelines](#pipelines)
- [Putting it all Together](#putting-it-all-together)

## Installation

Currently, `rox` can only be installed via `cargo`.  Install [Rust](https://www.rust-lang.org/tools/install) to get the entire `Rust` toolkit, including `cargo`.

Once that's done, run `cargo install rox` and then `rox --version` to verify that the installation succeeded.

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

Pipelines are the canonical way to chain together multiple tasks into a single execution unit. They also support parallel execution but it is up to the user to ensure that the tasks can be safely executed in parallel.

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
    description: "This task isn't callable directly from the CLI!"
    hide: true

```

## Upcoming Features

- Supports Monorepos via `workdir` specification
- Support Definitions in Multiple Files
- Thorough Testing (using [nextest](https://nexte.st/))

## Releasing

`Rox` is released by running `cargo release` locally.

Steps to Release:

1. `cargo install cargo-release` (if not already installed)
2. `cargo release [major|minor|patch] --execute`
