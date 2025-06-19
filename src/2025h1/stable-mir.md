# Publish first version of StableMIR on crates.io

| Metadata           |                                    |
| :--                | :--                                |
| Point of contact   | @celinval                          |
| Teams              | <!-- TEAMS WITH ASKS -->           |
| Task owners        | <!-- TASK OWNERS -->               |
| Status             | Accepted                           |
| Zulip channel      | [#project-stable-mir][channel]     |
| Tracking issue     | [rust-lang/rust-project-goals#266] |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/320896-project-stable-mir

## Summary

Publish StableMIR crate(s) to crates.io to allow tool developers to create applications on the top of the Rust compiler,
and extract code information from a compiled Rust crate and their dependencies without using compiler internal APIs.

## Motivation

In the past couple of years we have introduced a more stable API, named StableMIR, to the Rust compiler
to enable tool developers to analyze and extract information from compiled Rust crates without directly depending on compiler internals.
By publishing StableMIR crate(s) to crates.io, we can provide a reliable interface that enables developers to build analysis tools,
development environments, and other applications that work with Rust code while being insulated from internal compiler changes.

Publishing these crates through crates.io will make them easily accessible to the broader Rust community
and establish a foundation for building a robust ecosystem of development tools.
This will benefit the entire Rust ecosystem by enabling developers to create sophisticated tooling such as static analyzers,
linters, and development environments that can work reliably across different Rust compiler versions.
Besides stability, users will be able to rely on semantic versioning to track and adapt to changes,
reducing the existing maintenance burden for these developers.

### The status quo

In the past couple of years we have introduced a more stable API, named StableMIR, to the Rust compiler.
This API provides tool developers more stability and predictability, reducing the maintenance cost,
as well as providing a smaller surface API to reduce the ramp-up time for new developers.

However, StableMIR consumption model is still similar to any other internal compiler crate.
It doesn't have any explicit version, and it must be imported using an `extern crate` statement.

### The next 6 months

The first task is to restructure the relationship between `stable-mir` and `rustc_smir` crates,
eliminating existing dependencies on the `stable-mir` crate.

This will be followed by forking the `stable-mir` crate into its own repository,
where we'll implement CI jobs designed to detect any breaking changes that might occur due to compiler updates.

Once the structural changes are complete, we'll shift our attention to documentation and publication.
This includes creating comprehensive developer documentation that covers maintenance procedures for both crates,
ensuring future maintainers have clear guidelines for updates and compatibility management.

The final step will be publishing the newly refactored and documented version of stable-mir to crates.io,
making it readily available for tool developers in the Rust ecosystem.

### The "shiny future" we are working towards

By establishing a stable and well-documented interface,
we would like to empower developers to build a rich tooling ecosystem for Rust that can be
maintained in parallel with the Rust compiler's development.

This parallel development model ensures that tools can evolve alongside Rust itself,
fostering innovation and reducing bottlenecks.

## Design axioms

- Enable tool developers to implement sophisticated analysis with low maintenance cost.
- Do not compromise the development and innovation speed of the rust compiler.
- Crates should follow semantic versioning.

## Ownership and team asks


| Task                         | Owner(s) or team(s)            | Notes |
|------------------------------|--------------------------------|-------|
| Discussion and moral support | ![Team][] [compiler]           |       |
| Implementation               | @celinval                      |       |
| Standard reviews             | ![Team][] [project-stable-mir] |       |
| Fork configuration           | Help needed                    |       |
| Documentation                | Help needed                    |       |
| Publish crate                | @celinval                      |       |

### Definitions

Definitions for terms used above:

* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.

## Frequently asked questions