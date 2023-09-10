# Rox

Composable Dev Commands inspired by [Nox](https://nox.thea.codes/en/stable/) and Make

Rox gives you the ability to build your own CLI using YAML files, dynamically adding them to the `rox` command list. It's built with Rust, giving you fast startup times and cross-platform compabitility. Being both performant and flexible makes it easier for dev teams to standardize their workflows without writing endless "glue" scripts.

The subcommands and their help messages are automatically populated at runtime from the `name` and `description` of each `target`.

![Rox Help](assets/help_screenshot.png "Help Screenshot")

See the [example_rox.yml](example_rox.yml) for an idea of the planned end-state of syntax and functionality!

## Getting Started

### Requirements

Until a proper release pipeline is set up, you'll need `cargo` to get things up and running. Once you've got that installed, clone this repo and run `cargo build`.

To play around further, you can either run `cargo run help` or execute the built binary within the `target/debug/` dir.

## Features

- [x] Enforce Version Requirements (by doing semver-style version comparison)
- [x] Enforce that certain files exist (i.e. a `.env` file)
- [x] Show elapsed time all commands
- [x] Command dependencies (pre/post targets)
- [ ] Parametrize Commands
- [ ] Supports Parallelization
- [ ] Support passing inputs/outputs
- [ ] Support definitions in multiple files
- [ ] First-Class support for Git Operations?
