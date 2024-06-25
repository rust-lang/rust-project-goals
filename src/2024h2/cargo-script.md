# cargo-scrpt

| Metadata | |
| --- | --- |
| Owner(s) | epage |
| Teams | Cargo, Lang |
| Status | Accepted in [rust-lang/rust-project-goals#22](https://github.com/rust-lang/rust-project-goals/issues/22) |

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

### The next few steps

With [RFC 3502][] and [RFC 3503][] approved, the next steps are being tracked in [#12207](https://github.com/rust-lang/cargo/issues/12207).

[RFC 3502]: https://github.com/rust-lang/rfcs/pull/3502
[RFC 3503]: https://github.com/rust-lang/rfcs/pull/3503

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

## Ownership and other resources

**Owner:** epage

### Support needed from the project

- Review support from T-compiler.  Worse case, some mentorship as well.
- Lang has already approved [RFC 3503][] but other questions may come up.

## Outputs and milestones

### Outputs

Support for cargo scripts on stable.

### Milestones

- [x] Prototype
- [x] RFC
- [ ] Feature complete
- [ ] Stable

## Frequently asked questions
