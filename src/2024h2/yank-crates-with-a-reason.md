# Provided reasons for yanked crates

| Metadata       |                                    |
|----------------|------------------------------------|
| Point of contact | @Rustin170506                      |
| Status         | Accepted                           |
| Tracking issue | [rust-lang/rust-project-goals#101] |
| Zulip channel  | N/A                                |
## Summary

Over the next 6 months, we will add support to the registry yank API for providing a reason when a crate is yanked. This reason can then be displayed to users. After this feature has been up and running for a while, we'll open it up to Cargo to support filling in the reason for yanking.

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

The primary goal for the next 6 months is to add support to the registry's [yank API].

After that, next steps include (these can be done in many different orders):

* add support on the browser frontend for giving a reason
* add support on the cargo CLI for giving a reason
* add reason to the index
* add support on the cargo CLI for showing the reason

[yank API]: https://doc.rust-lang.org/cargo/reference/registry-web-api.html#yank

## Design axioms

When considering this feature, we need to balance our desire for a perfect, structured yank message with a usable, easy-to-use yank message. We need to start with this feature and leave room for future extensions, but we shouldn't introduce complexity and support for all requirements from the start.

## Ownership and team asks

**Owner:**

* @Rustin170506: wearing my crates.io team member's hat
* @Rustin170506: wearing my Cargo regular contributor's hat

| Task                                    | Owner(s) or team(s)            | Notes |
|-----------------------------------------|--------------------------------|-------|
| Implementation                          | @Rustin170506                  |       |
| Standard reviews                        | ![Team][] [crates-io]          |       |
| Deploy to production                    | ![Team][] [crates-io]          |       |
| Author RFC                              | @Rustin170506                  |       |
| RFC decision                            | ![Team][] [cargo], [crates-io] |       |
| Implementation in Cargo side            | @Rustin170506                  |       |
| Inside Rust blog post inviting feedback | @Rustin170506                  |       |
| Stabilization decision                  | ![Team][] [cargo]              |       |

[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red

## Frequently asked questions

### What might we do next?

We could start with plain text messages, but in the future we could consider designing it as structured data. This way, in addition to displaying it to Cargo users, we can also make it available to more crates-related platforms for data integration and use.