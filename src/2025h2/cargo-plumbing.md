# Prototype a new set of Cargo "plumbing" commands

| Metadata         |                                    |
|:-----------------|:-----------------------------------|
| Point of contact | @epage                             |
| Status           | Proposed                           |
| Needs            | Contributor                        |
| Zulip channel    | N/A                                |
| Tracking issue   | [rust-lang/rust-project-goals#264] |
| [cargo] champion | @epage                             |

## Summary

Create a third-party cargo subcommand that has "plumbing" (programmatic)
subcommands for different phases of Cargo operations to experiment with what
Cargo should integrate.

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

### The next 6 months

Continue on the third-party subcommand to experiment with plumbing commands ([source](https://github.com/crate-ci/cargo-plumbing)).

A build in Cargo can roughly be split into
1. Locate project
2. Read the manifests for a workspace
3. Read lockfile
4. Lock dependencies
5. Write lockfile
6. Resolve features
7. Plan a build, including reading manifests for transitive dependencies
8. Execute a build
9. Stage final artifacts

These could serve as starting points for experimenting with plumbing commands.
Staging of final artifacts may not be worth having a dedicated command for.
This is exclusively focused on build while other operations may be of interest to users.
We can evaluate those commands in the future as they tend to still build off of these same core primitives.

At minimum, later commands in the process would accept output from earlier commands,
allowing the caller to either replace commands (e.g. custom dependency resolver)
or customize the output (e.g. remove `dev-dependencies` from manifests).

Encapsulating stabilized file formats can serve as a starting point for output
schemas as we already output those and have to deal with stability guarantees
around these.

Between planning a build and executing a build is likely to look like
`--unit-graph` and a plan will need to be put forward for how to work through
the open issues.
There will likely be similar issues for any other output that can't leverage existing formats.

Cargo's APIs may not be able to expose each of these stages and work may need to be done to adapt it to support these divisions.

The performance of piping output between these commands may be sub-par, coming from a combination of at least
- Cargo's APIs may require doing more work than is needed for these stages
- Cargo focuses on json for programmatic output which may prove sub-par (see also [zulip](https://rust-lang.zulipchat.com/#narrow/channel/246057-t-cargo/topic/.60cargo.20metadata.60.20performance/near/476523460))
- Cargo's serde structures may not be optimized
- If customizing only a single step in this process,
  requiring serializing and deserializing through all of the other stages may be superfluous

Low hanging or egregious bottlenecks may need to be addressed.
Otherwise, performance should wait on user feedback.

A schema evolution plan will need to be considered with the design of the schema.
How Cargo deals with evolution of existing output could serve as potential starting points:
- `Cargo.toml` (generated by `cargo package`)  should still be readable by `cargo` versions within the specified `package.rust-version`
  - In the absence of a `package.rust-version`, `Cargo.toml` should only represent features the user explicitly used or optional features that were always allowed on stable `cargo`
- `Cargo.lock` (generated by most commands) is strictly versioned: all versions of Cargo should output a lockfile that works in all other versions of Cargo for that given version and changing Cargo versions should not cause the output to change
  - Cargo bumps the default format version after it has been stabilized for a "sufficient period of time"
  - The default is capped by what is supported by the lowest `package.rust-version` in the workspace
- `cargo metadata --format-version`: defaults to "latest" with a warning
  - We attempt to follow the same practice as `Cargo.toml`
- `--message-format`: no versioning currently
  - We attempt to follow the same practice as `Cargo.toml`

### The "shiny future" we are working towards

- Collect user feedback on these commands and iterate on them for eventual inclusion into Cargo
- Evaluate refactoring Cargo to better align with these plumbing commands to have better boundaries between subsystems
- Evaluate splitting the `cargo` `[lib]` into crates for each of these plumbing commands as smaller, more approachable, more "blessed" Rust APIs for users to call into

## Design axioms

- The changes to Cargo should not impede the development of Cargo
- The schemas and planned evolution should not impede the development of Cargo
- The plumbing commands should be focused on solving expected or known needs, avoiding speculation.

## Ownership and team asks

**Owner:** *Identify a specific person or small group of people if possible, else the group that will provide the owner. GitHub user names are commonly used to remove ambiguity.*

*This section defines the specific work items that are planned and who is expected to do them. It should also include what will be needed from Rust teams. The table below shows some common sets of asks and work, but feel free to adjust it as needed. Every row in the table should either correspond to something done by a contributor or something asked of a team. For items done by a contributor, list the contributor, or ![Heap wanted][] if you don't yet know who will do it. For things asked of teams, list ![Team][] and the name of the team. The things typically asked of teams are defined in the [Definitions](#definitions) section below.*

| Task                                    | Owner(s) or team(s)      | Notes |
|-----------------------------------------|--------------------------|-------|
| Discussion and moral support            | ![Team][] [cargo]        |       |
| Implementation                          | ![Help wanted][]         |       |
| Optimizing Cargo                        | ![Help wanted][], @epage |       |
| Inside Rust blog post inviting feedback | @epage                   |       |

### Definitions

Definitions for terms used above:

* *Discussion and moral support* is the lowest level offering, basically committing the team to nothing but good vibes and general support for this endeavor.
* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* *Prioritized nominations* refers to prioritized lang-team response to nominated issues, with the expectation that there will be *some* response from the next weekly triage meeting.
* *Dedicated review* means identifying an individual (or group of individuals) who will review the changes, as they're expected to require significant context.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

## Frequently asked questions
