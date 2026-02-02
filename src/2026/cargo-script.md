# Stabilize cargo-script

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @epage                                                                           |
| Status           | Proposed                                                                         |
| Flagship         | Higher-level Rust                                                                |
| Tracking issue   | [rust-lang/rust-project-goals#119]                                               |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |
| Stabilization    | true                                                                             |

## Summary

Stabilize support for "cargo script", the ability to have a single file that contains both Rust code and a `Cargo.toml`.

## Motivation

Being able to have a Cargo package in a single file can reduce friction in development and communication,
improving bug reports, educational material, prototyping, and development of small utilities.

### The status quo

Today, at minimum a Cargo package is at least two files (`Cargo.toml` and either `main.rs` or `lib.rs`).
The `Cargo.toml` has several required fields.

To share this in a bug report, people resort to
- Creating a repo and sharing it
- A shell script that cats out to multiple files
- Manually specifying each file
- Under-specifying the reproduction case (likely the most common due to being the easiest)

To create a utility, a developer will need to run `cargo new`, update the
`Cargo.toml` and `main.rs`, and decide on a strategy to run this (e.g. a shell
script in the path that calls `cargo run --manifest-path ...`).

Cargo has had unstable Cargo script support for years.
New, unstable syntax has been added and gone through rounds of testing.
A [stabilization report](https://github.com/rust-lang/rust/pull/148051)
exists for frontmatter syntax but work is needed to finish going through that process and then on Cargo's side.

### The next 6 months

Work with T-lang, T-cargo, and other affected teams as we go through the stabilization process.

| Task        | Owner(s) | Notes |
| ----------- | -------- | ----- |
| T-rustdoc decide on frontmatter in doctests | *owner*  |       |
| Implement behavior for frontmatter in doctests | *owner*  |       |

### The "shiny future" we are working towards

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Small         | Stabilization process                   |
| [compiler] | Small         | Reviewing any further compiler changes  |
| [lang]     | Small         | Stabilization discussions               |
| [rustdoc]  | Small         | Design decision and PR review |

## Frequently asked questions
