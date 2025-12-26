# Stabilize cargo-script

| Metadata       |                                    |
| ---            | ---                                |
| Point of contact | @epage                           |
| Status         | Accepted                           |
| Tracking issue | [rust-lang/rust-project-goals#119] |
| Zulip channel  | N/A                                |

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
- Under-specifying the reproduction case (likely the most common due to being the eaisest)

To create a utility, a developer will need to run `cargo new`, update the
`Cargo.toml` and `main.rs`, and decide on a strategy to run this (e.g. a shell
script in the path that calls `cargo run --manifest-path ...`).

### The next six months

The support is already implemented on nightly.
The goal is to stabilize support.
With [RFC #3502] and [RFC #3503] approved, the next steps are being tracked in [rust-lang/cargo#12207].

At a high-level, this is
- Add support to the compiler for the frontmatter syntax
- Add support in Cargo for scripts as a "source"
- Polish

### The "shiny future" we are working towards

## Design axioms

- In the trivial case, there should be no boilerplate.  The boilerplate should scale with the application's complexity.
- A script with a couple of dependencies should feel pleasant to develop without copy/pasting or scaffolding generators.
- We don't need to support everything that exists today because we have multi-file packages.

[da]: ../about/design_axioms.md

## Ownership and team asks

**Owner:** epage

This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams.

* Subgoal:
    * Describe the work to be done and use `↳` to mark "subitems".
* Owner(s) or team(s):
    * List the owner for this item (who will do the work) or ![Help wanted][] if an owner is needed.
    * If the item is a "team ask" (i.e., approve an RFC), put ![Team][] and the team name(s).
* Status:
    * List ![Help wanted][] if there is an owner but they need support, for example funding.
    * Other needs (e.g., complete, in FCP, etc) are also fine.

| Task                   | Owner(s) or team(s)      | Notes |
| ---------------------- | ------------------------ | ----- |
| Implementation         | @epage                   |       |
| Stabilization decision | ![Team][] [lang] [cargo] |       |