# Rox

![crates.io](https://img.shields.io/crates/v/rox-cli.svg)
[![CI Checks](https://github.com/ThomasLaPiana/rox/actions/workflows/checks.yml/badge.svg)](https://github.com/ThomasLaPiana/rox/actions/workflows/checks.yml)

Composable build tool inspired by [Nox](https://nox.thea.codes/en/stable/), Make & [cargo-make](https://github.com/sagiegurari/cargo-make)

Rox gives you the ability to build your own devtools CLI using YAML files. Tasks and Pipelines are dynamically added to the CLI as subcommands at runtime. The flexibility of `rox` intends to makes it easier for dev teams to standardize their workflows without writing endless "glue" scripts.

The subcommands and their help messages are automatically populated at runtime from the `name` and `description` of each `task` or `pipeline`.

See [gameslog](https://github.com/ThomasLaPiana/gameslog-rs) for an example of usage in a real project.

## Table of Contents

- [Why Rox?](#why-rox)
- [Installation](#installation)
- [Roxfile Syntax](#roxfile-syntax)
  - [Version Requirements](#version-requirements)
  - [File Requirements](#file-requirements)
  - [Templates](#templates)
  - [Tasks](#tasks)
  - [Pipelines](#pipelines)
  - [Putting it all Together](#putting-it-all-together)
- [Usage Examples](#usage-examples)

## Why Rox?

Rox was created for the purpose of making building and developing applications easier. It is designed to focus on extensiblity, performance, and documentation. Here are a few of the key features that help Rox achieve that goal:

- __Dynamically Generated CLI__: Rox's tasks and pipelines are dynamically added as subcommands to the CLI at runtime. Configuration is handled entirely in YAML files.
- __Powerful Primitives__: Using a combination of Rox's primitives (`Tasks`, `Pipelines` and `Templates`) it is possible to handle virtually any use-case with elegance and minimal boilerplate.
- __Documentation as a First-Class Feature__: Names and descriptions are automatically injected into the CLI at runtime, so your `help` command is always accurate. This helps developers understand what various tasks and pipelines do without needing to dive into the code.
- __Performant__: Minimal overhead and native executables for a variety of architectures and operating systems. (At time of writing, the `rox task help` command ran in ~5 milliseconds for the given `roxfile.yml`, which includes both startup and file parsing time!)
- __Efficient__: By utilizing pipelines and parallel execution, developers are empowered to make use of multi-core machines to speed up build and development tasks.
- __User-Friendly__: Task results are shown to the user in an easy-to-consume table format along with useful metadata. This makes debugging easier, and shows potential bottlenecks in build steps.

## Installation

Rox can be installed via binaries provided with each release [here](https://github.com/ThomasLaPiana/rox/releases). As an alternative, it can also be installed via `cargo` with `cargo install rox-cli`.

## Roxfile Syntax

Rox requires a `YAML` file with the correct format and syntax to be parsed into a CLI. This file is expected to be at `./roxfile.yml` by default but that can be overriden with the `-f` flag at runtime.

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

Pipelines are the canonical way to chain together multiple tasks into a single unit of execution. Note that `stages` object expects a list of lists, which we'll expand upon below.

```yaml
pipelines: 
  - name: example-pipeline
    description: Composes a few tasks
    stages: [["task-a", "task-b", "task-c"]]
```

To make execution more efficient, Pipelines support a simple DAG definition syntax that allows `tasks` _within_ the same stage to be executed in parallel. This gives user more fine-grained control over how multiple tasks are executed while still keeping the syntax relatively lightweight. Parallel execution is not used by default, and requires using the `--parallel` or `-p` flag on invocation.

The `stages` field expects a list of lists to facilitate this. Each `stage` is like a small pipeline in and of itself, and each stage's tasks must all finish executing before work starts on the next stage.

In the following example, the parallel execution pattern would look like this:

1. Tasks `a` is executed
1. Tasks `b` and `c` are executed, potentially in parallel
1. Tasks `e` and `d` are executed, potentially in parallel.
1. Finally, task `f` is executed.

```yaml
pipelines: 
  - name: example-pipeline
    description: Composes a few tasks
    stages:
      - ["task-a"]
      - ["task-b", "task-c"]
      - ["task-e", "task-d"]
      - ["task-f"]
```

### Putting it all together

Now that we've seen each individual piece of the Rox puzzle, we can put them all together into a full `roxfile`. See the [example roxfile.yml](roxfile.yml) in this repo for a working example!

## Video Walkthrough

The following is a brief video walkthrough of what using `Rox` looks like in a real project:

## Releasing

`Rox` is released by running `cargo release` locally.

Steps to Release:

1. Make sure that all desired changes are pushed up and merged to `main`
1. `cargo install cargo-release` (if not already installed)
1. `cargo release [major|minor|patch] --execute` - Updates the `Cargo.toml`, commits and pushes the change, and then publishes the crate to <crates.io>
1. `cargo release tag --execute` - Creates a git tag with the same version as the `Cargo.toml`
1. `cargo release push --execute` - Pushes the git tag
1. Finally, a CI job is automatically triggered to build and upload the release assets

## Upcoming Features

The following is a list of roughly-planned features that are on the roadmap for Rox.

- Platform-specific tasks - Windows users rejoice
- Replace `split` with `split_take` - Allow users to choose exactly which split part to use for their version checks
- Write command outputs to local log files
