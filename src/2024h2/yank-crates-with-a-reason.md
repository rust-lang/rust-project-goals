# Administrator-provided reasons for yanked crates

| Metadata |                      |
| -------- | -------------------- |
| Owner(s) | [hi-rustin]          |
| Teams    | [crates.io], [Cargo] |
| Status   | WIP                  |

[hi-rustin]: https://github.com/hi-rustin
[Cargo]: https://www.rust-lang.org/governance/teams/dev-tools#team-cargo
[crates.io]: https://www.rust-lang.org/governance/teams/dev-tools#team-crates-io

## Summary

Over the next 6 months, we will first implement a feature in crates.io that asks for a reason when an administrator yanks a crate. After this feature has been up and running for a while, we'll open it up to Cargo to support filling in the reason for yanking, making it an optional parameter of the registry yank API.

## Motivation

When a crate is updated to address a critical issue—such as a fix for a soundness bug or a security vulnerability—it is beneficial to yank previous versions and prompt users to upgrade with a yank reason. Additionally, if a crate is renamed or deprecated, the yank message can provide guidance on the new recommended crate or version. This ensures that users are aware of necessary updates and can maintain the security and stability of their projects.

### The status quo

We came up with [this need](https://github.com/rust-lang/cargo/issues/2608) eight years ago, but it was never implemented.

This feature has the following potential use cases:

1. When a crate is fixed because it will be broken in the next version of the compiler (e.g. a soundness fix or bug fix) then the previous versions can be yanked and nudge users forward.
2. If a crate is fixed for a security reason, the old versions can be yanked and the new version can be suggested.
3. If a crate is renamed (or perhaps deprecated) to another then the yank message can indicate what to do in that situation.

Additionally, if we can persist this information to the crates.io index, we can make it available as meta-information to other platforms, such as security platforms like RustSec.


### The next 6 months

* Implementing basic prototypes in crates.io
* Trial run at crates.io for a while
* Support for yank messages on the Cargo command line(unstable)
* Stabilize this parameter and use it as a standard parameter for the registry [yank API]

[yank API]: https://doc.rust-lang.org/cargo/reference/registry-web-api.html#yank

## Design axioms

When considering this feature, we need to balance our desire for a perfect, structured yank message with a usable, easy-to-use yank message. We need to start with this feature and leave room for future extensions, but we shouldn't introduce complexity and support for all requirements from the start.

## Ownership and other resources

**Owner:**

* [hi-rustin]: wearing my crates.io team member's hat
* [hi-rustin]: wearing my Cargo regular contributor's hat

| Subgoal | Owner(s) or team(s) | Notes |
| ----------------------------------------------------------- | ------------------------------ | ------ |
| Yank crates with a reason                                   |                                |        |
| ↳ Implementation in crates.io side(only for administrators) | [hi-rustin]                    |        |
| ↳ Standard reviews                                          | ![Team][] [crates.io]          |        |
| ↳ Try it out in crates.io                                   | ![Team][] [crates.io]          |        |
| ↳ Author RFC                                                | [hi-rustin]                    |        |
| ↳ Approve RFC                                               | ![Team][] [Cargo], [crates.io] |        |
| ↳ Implementation in Cargo side                              | [hi-rustin]                    |        |
| ↳ Inside Rust blog post inviting feedback                   | [hi-rustin]                    |        |
| ↳ Stabilization decision                                    | ![Team][] [Cargo]              |        |

[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

## Frequently asked questions

### What might we do next?

We could start with plain text messages, but in the future we could consider designing it as structured data. This way, in addition to displaying it to Cargo users, we can also make it available to more crates-related platforms for data integration and use.
