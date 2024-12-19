# Begin resolving `cargo-semver-checks` blockers for merging into cargo

| Metadata       |                                    |
| ---            | ---                                |
| Point of contact | @obi1kenobi                        |
| Teams          | [cargo]                            |
| Status         | Accepted                           |
| Tracking issue | [rust-lang/rust-project-goals#104] |


## Summary

Design and implement `cargo-semver-checks` functionality that lies on the critical path for merging the tool into cargo itself.

## Motivation

Cargo assumes that all packages adhere to semantic versioning (SemVer).
However, SemVer adherence is quite hard in practice: [research shows][semver-study] that accidental SemVer violations are relatively common (lower-bound: in 3% of releases) and happen to Rustaceans of all skill levels.
Given the significant complexity of the Rust SemVer rules, improvements here require better tooling.

`cargo-semver-checks` is a linter for semantic versioning (SemVer) in Rust.
It is broadly adopted by the Rust community, and the [cargo] team has expressed interest in merging it into cargo itself as part of the existing `cargo publish` workflow.
By default, `cargo publish` would require SemVer compliance, but offer a flag (analogous to the `--allow-dirty` flag for uncommitted changes) to override the SemVer check and proceed with publishing anyway.

The [cargo] team has identified [a set of milestones and blockers][merge-blockers] that must be resolved before `cargo-semver-checks` can be integrated into the `cargo publish` workflow.
Our goal here is to resolve one of those blockers (cargo manifest linting), and chart a path toward resolving the rest in the future.

[semver-study]: https://predr.ag/blog/semver-violations-are-common-better-tooling-is-the-answer/
[merge-blockers]: https://github.com/obi1kenobi/cargo-semver-checks/issues/61

### The status quo

Work in three major areas is required to resolve the [blockers][merge-blockers] for running `cargo-semver-checks` as part of `cargo publish`:
- Support for cargo manifest linting, and associated CLI changes
- Checking of cross-crate items
- SemVer linting of type information

Fully resolving all three areas is likely a 12-24 month undertaking, and beyond the scope of this goal on its own.
Instead, this goal proposes to accomplish intermediate steps that create immediate value for users and derisk the overall endeavor, while needing only "moral support" from the [cargo] team as its only requirement.

#### Cargo manifest linting

Package manifests have SemVer obligations: for example, removing a feature name that used to exist is a major breaking change.

Currently, `cargo-semver-checks` is [not able to catch such breaking changes][manifest-lints].
It only draws information from a package's rustdoc JSON, which does not include the necessary manifest details and does not have a convincing path to doing so in the future.
Design and implementation work is required to allow package manifests to be linted for breaking changes as well.

This "rustdoc JSON only" assumption is baked into the `cargo-semver-checks` CLI as well, with options such as `--baseline-rustdoc` and `--current-rustdoc` that allow users to lint with a pre-built rustdoc JSON file instead of having `cargo-semver-checks` build the rustdoc JSON itself.
Once manifest linting is supported, users of such options will need to somehow specify a `Cargo.toml` file (and possibly even a matching `Cargo.lock`) in addition to the rustdoc JSON.
Additional work is required to determine how to evolve the CLI to support manifest linting and future-proof it to the level necessary to be suitable for stabilizing as part of cargo's own CLI.

[manifest-lints]: https://github.com/obi1kenobi/cargo-semver-checks/issues/48

#### Checking of cross-crate items

Currently, `cargo-semver-checks` performs linting by only using the rustdoc JSON of the target package being checked.
However, the public API of a package may expose items from other crates.
Since rustdoc no longer inlines the definitions of such foreign items into the JSON of the crate whose public API relies on them, `cargo-semver-checks` [cannot see or analyze them][cross-crate-items].

This causes a massive number of false-positives ("breakage reported incorrectly") and false-negatives ("lint for issue X fails to spot an instance of issue X").
In excess of 90% of real-world false-positives are traceable back to a cross-crate item, as measured by our [SemVer study][semver-study]!

For example, the following change is not breaking but `cargo-semver-checks` will incorrectly report it as breaking:
```rust
// previous release:
pub fn example() {}

// in the new release, imagine this function moved to `another_crate`:
pub use another_crate::example;
```
This is because the rustdoc JSON that `cargo-semver-checks` sees indeed *does not contain* a function named `example`.
Currently, `cargo-semver-checks` is incapable of following the cross-crate connection to `another_crate`, generating its rustdoc JSON, and continuing its analysis there.

Resolving this limitation will require changes to how `cargo-semver-checks` generates and handles rustdoc JSON, since the set of required rustdoc JSON files will no longer be fully known ahead of time.
It will also require CLI changes in the same area as the changes required to support manifest linting.

While there may be other challenges on rustc and rustdoc's side before this feature could be fully implemented, we consider those out of scope here since there are [parallel efforts to resolve them][parallel-efforts].
The goal here is for `cargo-semver-checks` to have its own story straight and do the best it can.

[cross-crate-items]: https://github.com/obi1kenobi/cargo-semver-checks/issues/638
[parallel-efforts]: https://github.com/rust-lang/compiler-team/issues/635

#### SemVer linting of type information

Currently, `cargo-semver-checks` lints cannot represent or examine type information.
For example, the following change is breaking but `cargo-semver-checks` will not detect or report it:
```rust
// previous release:
pub fn example(value: String) {}

// new release:
pub fn example(value: i64) {}
```
Analogous breaking changes to function return values, struct fields, and associated types would also be missed by `cargo-semver-checks` today.

The main difficulty here lies with the expressiveness of the Rust type system. For example, none of the following changes are breaking:
```rust
// previous release:
pub fn example(value: String) {}

// new release:
pub fn example(value: impl Into<String>) {}

// subsequent release:
pub fn example<S: Into<String>>(value: S) {}
```
Similar challenges exist with lifetimes, variance, trait solving, `async fn` versus `fn() -> impl Future`, etc.

While there are some promising preliminary ideas for resolving this challenge, more in-depth design work is necessary to determine the best path forward.

### The next 6 months

Three things:
- Implement cargo manifest linting
- Implement CLI future-proofing changes, with manifest linting and cross-crate analysis in mind
- Flesh out a design for supporting cross-crate analysis and type information linting in the future

### The "shiny future" we are working towards

Accidentally publishing SemVer violations that break the ecosystem is never fun for anyone involved.

From a user perspective, we want a fearless `cargo update`: one's project should never be broken by updating dependences without changing major versions.

From a maintainer perspective, we want a fearless `cargo publish`: we want to prevent breakage, not to find out about it when a frustrated user opens a GitHub issue. Just like cargo flags uncommitted changes in the publish flow, it should also _quickly_ and _accurately_ flag breaking changes in non-major releases. Then the maintainer may choose to release a major version instead, or acknowledge and explicitly override the check to proceed with publishing as-is.

To accomplish this, `cargo-semver-checks` needs the ability to express more kinds of lints (including manifest and type-based ones), eliminate false-positives, and stabilize its public interfaces (e.g. the CLI). At that point, we'll have lifted the [main merge-blockers][merge-blockers] and we can consider making it a first-party component of cargo itself.

## Ownership and team asks

**Owner:** @obi1kenobi, as maintainer of `cargo-semver-checks`

I (@obi1kenobi) will be working on this effort. The only other resource request would be occasional discussions and moral support from the [cargo] team, of which I already have the privilege as maintainer of a popular cargo plugin.

| Task                                             | Owner(s) or team(s)     | Notes |
| ------------------------------------------------ | ----------------------- | ----- |
| Implementation of cargo manifest linting + CLI   | @obi1kenobi             |       |
| Initial design for cross-crate checking          | @obi1kenobi             |       |
| Initial design for type-checking lints           | @obi1kenobi             |       |
| Discussion and moral support                     | ![Team][] [cargo]       |       |

## Frequently asked questions

### Why not use semverver instead?

[Semverver][semverver] is a prior attempt at enforcing SemVer compliance, but has been deprecated and is no longer developed or maintained.
It relied on compiler-internal APIs, which are much more unstable than rustdoc JSON and required much more maintenance to "keep the lights on."
This also meant that semverver required users to install a specific nightly versions that were known to be compatible with their version of semverver.

While `cargo-semver-checks` relies on rustdoc JSON which is also an unstable nightly-only interface, its changes are much less frequent and less severe.
By using the [Trustfall query engine][trustfall], `cargo-semver-checks` can simultaneously support a range of rustdoc JSON formats (and therefore Rust versions) within the same tool.
On the maintenance side, `cargo-semver-checks` lints are written in a declarative manner that is oblivious to the details of the underlying data format, and do not need to be updated when the rustdoc JSON format changes.
This makes maintenance much easier: updating to a new rustdoc JSON format usually requires just a few lines of code, instead of "a few lines of code apiece in each of hundreds of lints."

[semverver]: https://github.com/rust-lang/rust-semverver
[trustfall]: https://github.com/obi1kenobi/trustfall