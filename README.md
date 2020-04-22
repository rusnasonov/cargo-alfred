[![Crates.io](https://img.shields.io/crates/v/cargo-alfred.svg)](https://crates.io/crates/cargo-alfred) [![Crates.io](https://img.shields.io/crates/d/cargo-alfred.svg)](https://crates.io/crates/cargo-alfred) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/rusnasonov/blob/master/LICENSE)

# cargo-alfred

This is [Cargo Subcomand](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands) which create [Alfred Workflow](https://www.alfredapp.com/workflows/) from Cargo project.

## Usage

Install with Cargo

`cargo install cargo-alfred`

Add `[package.metadata.alfred]` to project's `Cargo.toml` with following fields, see [example](https://github.com/rusnasonov/cargo-alfred/tree/master/example):

```
workflow_name = "example_workflow_name"
bundle_id = "example_bundle_id"
category = "Productivity"
created_by = "Ruslan Nasonov"
description = "example description"
name = "example_name"
keyword = "example"
title = "example_title"
include = ["README.md"]
```

Next run `cargo alfred` inside you project. That's it! You'll see `workflow_name.alfredworkflow` file inside project root directory.
