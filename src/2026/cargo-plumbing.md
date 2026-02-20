# Prototype a new set of Cargo "plumbing" commands

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @epage                                                                           |
| Status           | Proposed                                                                         |
| Needs            | Contributor                                                                      |
| Roadmap          | Building blocks                                                                  |
| Tracking issue   | [rust-lang/rust-project-goals#264]                                               |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

1. Refactor Cargo to allow hacks in
[proposed cargo-plumbing commands](https://github.com/crate-ci/cargo-plumbing)
to be removed ([cargo-plumbing#82](https://github.com/crate-ci/cargo-plumbing/issues/82)).
2. Round out proposed commands ([issues](https://github.com/crate-ci/cargo-plumbing/issues?q=is%3Aissue%20state%3Aopen%20label%3AA-new-subcommand))
3. Finalize the message formats ([cargo-plumbing#18](https://github.com/crate-ci/cargo-plumbing/discussions/18))

**Needs contributor:** This goal needs contributors to refactor Cargo internals, implement remaining plumbing commands, optimize performance, and iterate on output schemas. The work is primarily in [rust-lang/cargo](https://github.com/rust-lang/cargo) and [crate-ci/cargo-plumbing](https://github.com/crate-ci/cargo-plumbing). Estimated time commitment: TBD.

## Motivation

Cargo is a "porcelain" (UX) focused command and is highly opinionated which can work well for common cases.
However, as Cargo scales into larger applications, users need the ability to adapt Cargo to their specific processes and needs.

### The status quo

While most Cargo commands can be used programmatically, they still only operate at the porcelain level.
Currently, Cargo's plumbing commands are
- `cargo read-manifest`:
  - works off of a `Cargo.toml` file on disk
  - uses a custom json schema
  - deprecated
- `cargo locate-project`:
  - works off of a `Cargo.toml` file on disk
  - text or json output, undocumented json schema
  - uses a pre-1.0 term for package
- `cargo metadata`:
  - works off of `Cargo.toml`, `Cargo.lock` files on disk
  - uses a custom json schema
  - can include dependency resolution but excludes feature resolution
  - some users want this faster
  - some users want this to report more information
  - See also [open issues](https://github.com/rust-lang/cargo/issues?q=is%3Aissue%20state%3Aopen%20label%3ACommand-metadata)
- `cargo pkgid`:
  - works off of `Cargo.toml`, `Cargo.lock` files on disk
  - text output
- `cargo verify-project`:
  - works off of a `Cargo.toml` file on disk
  - uses a custom json schema
  - uses a pre-1.0 term for package
  - deprecated

There have been experiments for a plumbing for builds
- [`--build-plan`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-plan) attempts to report what commands will be run so external build tools can manage them.
  - The actual commands to be run is dynamic, based on the output of build scripts from build graph dependencies
  - Difficulty in supporting build pipelining
- [`--unit-graph`](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#unit-graph) reports the graph the build operates off of which corresponds to calls to the compiler and build scripts
  - Also provides a way to get the results of feature resolution

Thanks to GSoC, we now have [prototypes for some plumbing commands](https://github.com/crate-ci/cargo-plumbing).

### The next 6 months

Continue on the third-party subcommand to experiment with plumbing commands ([source](https://github.com/crate-ci/cargo-plumbing)).

| Task                                    | Owner(s) or team(s)      | Notes |
|-----------------------------------------|--------------------------|-------|
| Refactor cargo                          | ![Help wanted][]         |       |
| Implement remaining commands            | ![Help wanted][]         |       |
| Inside Rust blog post inviting feedback | @epage                   |       |
| Optimizing Cargo                        | ![Help wanted][], @epage |       |
| Iterate on schemas including schema evolution plan | ![Help wanted][]         |       |

See [2025h2 goal](../2025h2/cargo-plumbing.md) for more background.

### The "shiny future" we are working towards

- Collect user feedback on these commands and iterate on them for eventual inclusion into Cargo
- Evaluate refactoring Cargo to better align with these plumbing commands to have better boundaries between subsystems
- Evaluate splitting the `cargo` `[lib]` into crates for each of these plumbing commands as smaller, more approachable, more "blessed" Rust APIs for users to call into

## Team asks

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Small         | PR reviews for Cargo changes; design discussions |

## Frequently asked questions
