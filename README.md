# Rox

Composable Dev Commands inspired by [Nox](https://nox.thea.codes/en/stable/) and Make

Rox gives you the ability to build your own CLI using YAML files, but with a fast and lightweight Rust backend. This makes it performant and flexible, lettings development teams standardize their developer experience and commands without writing endless "glue" scripts.

See the [example_rox.yml](example_rox.yml) for an idea of the planned end-state of syntax and functionality!

## Features

- [x] Enforce Version Requirements (by doing semver-style version comparison)
- [x] Enforce that certain files exist (i.e. a `.env` file)
- [ ] Command dependencies (pre/post targets)
- [ ] First-Class support for Git Operations
- [ ] Parametrize Commands
- [ ] Support passing inputs/outputs
- [ ] Supports Parallelization
- [ ] Show elapsed time all commands
- [ ] Support definitions in multiple files
