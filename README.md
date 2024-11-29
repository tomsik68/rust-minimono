# Rust Minimono

This repository shows a possible layout of a Rust mini-monorepo.

This layout is a welcoming environment for all the things - libraries, binaries and even build tasks!

Goals of this:
1. Showcase that many things can be put together to form a meaningful whole.
2. Configure some widely-used helpful Rust tools.
3. Help the "Rustification" of build scripts by incorporating nice xtask jobs.
4. Make it very easy to onboard new collaborators.

Non-goals:
1. Prescribe that your company uses monorepo. Please use whatever floats your boat.

## Pre-configured tools

- [deny](https://github.com/EmbarkStudios/cargo-deny): lint dependencies
- [taplo](https://github.com/tamasfe/taplo): format Cargo.toml files
- [wipe](https://github.com/mihai-dinculescu/cargo-wipe): wipe target folder
- [msrv](https://github.com/foresterre/cargo-msrv): find actual MSRV for project

## top-level directories

### crates

Rust Crates that are part of this repository.

You can use `cargo xt new` to create a new crate in this repository.

### xtask

Implementation of xtask jobs.

# First-time setup

## Installing rustup

Rustup is a tool that manages different versions of Rust toolchain on your system.
Let's install that first by running the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For alternative installation options, please refer to [rustup.rs](https://rustup.rs/)

There's no need to re-run this command - rustup remains installed on your machine.

## Idempotent

```bash
cargo xtask ensure-installed
```

Note that rustup ensures you're using the right version of Rust as declared in rust-toolchain.toml.

This command may need to be re-run in case someone adds new tools to the set. The neat part is that re-running makes sure your environment is up-to-date.

# Other available tasks

Please run the following command to get an overview:

```bash
cargo xtask help
```

# Is `cargo xt` too long?

You can use a shell alias! Most shells will be okay with:

```bash
alias cx='cargo xt'
```
