# Prototype a new set of Cargo "plumbing" commands

| Metadata         |                                                                                  |
| :--------------- | -------------------------------------------------------------------------------- |
| Point of contact | @epage                                   |
| Status           | Proposed for mentorship                                                                         |
| Flagship         | Building blocks                                                                  |
| Tracking issue   | [rust-lang/rust-project-goals#264] |
| Zulip channel    | N/A (an existing stream can be re-used or new streams can be created on request) |

## Summary

1. Refactor Cargo to allow hacks in
[proposed cargo-plumbing commands](https://github.com/crate-ci/cargo-plumbing)
to be removed ([cargo-plumbing#82](https://github.com/crate-ci/cargo-plumbing/issues/82)).
2. Round out proposed commands ([issues](https://github.com/crate-ci/cargo-plumbing/issues?q=is%3Aissue%20state%3Aopen%20label%3AA-new-subcommand))
3. Finalize the message formats ([cargo-plumbing#18](https://github.com/crate-ci/cargo-plumbing/discussions/18))

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

> This section outlines what support you need from the Rust teams. For each team, identify the level of support you need:
>
> * Vibes: You don't need the team to do anything at all, but you do want to know they like your idea.
>     * *Example:* Prototyping a new feature on crates.io that you hope to eventually upstream.
>     * *Example:* Conducting research that might eventually become a language feature.
> * Small: You only need the team to do its routine activities.
>     * *Example:* A compiler change that will require a few small PRs to be reviewed.
>     * *Example:* Asking the lang team to approve a lint.
> * Medium: You need dedicated support from one person, but the rest of the team doesn't have to do much.
>     * *Example:* A compiler change that doesn't require any rearchitecting but 
>     * *Example:* Implementing a small, noncontroversial language feature.
> * Large: You need deeper review from the entire team.
>     * *Example:* Rearchitecting part of the compiler.
>     * *Example:* Implementing a complex language feature that will require design meetings.
>
> If you're not sure, leave it blank, the project goals team can help.
>
> "Vibes" and "Small" asks require someone on the team to "second" your goal; "Medium" and "Large" asks require a dedicated champion from the team. If you don't have a second or a champion, the project goals team will help you find them, don't worry about it.

| Team       | Support level | Notes                                   |
| ---------- | ------------- | --------------------------------------- |
| [cargo]    | Small         | PR reviews for Cargo changes; design discussions |

## Frequently asked questions
