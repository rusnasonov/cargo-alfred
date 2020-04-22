# cargo-alfred

This is [Cargo Subcomand](https://github.com/rust-lang/cargo/wiki/Third-party-cargo-subcommands) which create [Alfred Workflow](https://www.alfredapp.com/workflows/) from Cargo project.

## Usage

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
