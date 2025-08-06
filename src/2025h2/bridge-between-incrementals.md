# Bridge between incremental caches for all tools

| Metadata         |                                                                                  |
|:-----------------|----------------------------------------------------------------------------------|
| Point of contact | @blyxyas                                                                         |
| Teams            | <!-- TEAMS WITH ASKS -->                                                         |
| Task owners      | @blyxyas                                                                         |
| Status           | Proposed                                                                         |
| Tracking issue   |                                                                                  |
| Zulip channel    |                                                                                  |

## Summary

The goal of this project goal is to make a compatibility layer to share incremental compilation information between
`cargo check`, `cargo build`, and any other external tool that might benefit from sharing incremental information
such as `cargo clippy`.

## Motivation

Redoing work between `cargo check`, `cargo build` and other commands such as `cargo clippy` is one a pain point in
compiler performance. Being able to reuse this information would speed up by a lot both
CI/CD pipelines as well as the developer feedback loop. Making Rust a more pleasing experience to work with and
reducing the inherent "time punishment" when handling large projects.

### The status quo

Currently running Rust on CI largely consists of unconnected stages for checking, linting, and building/testing. In a
scenario where time is literally money we're redoing a very large amount of work for lack of a better system. On local
user's machines, we're producing the same information several times even with persistent incremental directories.

As an example, running `cargo check` and then `cargo clippy` will double the `target` directory size.

### The next 6 months

The plan to continue looks something like this:

- Identify what information is individual to each workflow and which is repeated information.
- Create an information protocol that all Rust builds produce containing module metadata and compilation information.
- And adapt existing APIs in order to facilitate as much as possible information sharing between different tools with different needs.

### The "shiny future" we are working towards

The ideal future would be one in which all Rust compilation-adjacent activities can share a robust common ground, extensible
enough for any tool to privately use. With the end goal of speeding up compiler performance on incremental, linting performance
and shrinking incremental sizes.

In this ideal scenario, any existing or future team / even third-party project (although not a priority) could hook up their
compilation-adjacent project to an existing incremental compilation directory and gather all useful information. As a result,
no matter how many different activities or tools the user uses. The incremental directory size would not be linear but closer
to logarithmic.

As a technical detail, in this ideal scenario each compiler-adjacent activity would emit and request a subset of data such as
module information or targets, available for all other activities to extend and/or use.

## Design axioms

- **Atomic and usable without external requirements**, the protocol should be as open-ended as possible in order to facilitate collaboration between tools.
- **Activities operate on a request - emit scheme**, every and any activity should be able to request or emit compilation information via a straightforward way (yet to determine)
- **Link all information to module data**, if a module data dependency doesn't change, a different activity shouldn't be reran. If an activity A is ran after an activity B, don't force A to be reran because B has changed unrelated hashes.

## Ownership and team asks

| Task                           | Owner(s) or team(s)                         | Notes |
|--------------------------------|---------------------------------------------|-------|
| Initial design proposal        | @blyxyas                                    |       |
| Discussion on the design       | [Help Wanted, wg-incremental, cargo?]       |       |
| Implementation                 | @blyxyas                                    |       |
| Revision of the implementation | [Help Wanted], wg-incremental?              |       |

### Definitions

For definitions for terms used above, see the [About > Team Asks](https://rust-lang.github.io/rust-project-goals/about/team_asks.html) page.

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

### Why is the cargo team being mentioned?

Being that Cargo is by far the most popular interface into the Rust compiler, I think that they should be aware of this change in architecture. Maybe they have any requirement they'd like to voice out or concern they'd like to iron out before proceeding with any testing / design.

### How would this model deal with `cargo check` run after partial builds?

> A check and regular build will produce a different crate hash, so if you do `cargo check; cargo build -p some_dep; cargo check` you are now forced to rebuild everything that depends on `some_dep` as the crate hash of `some_dep` changed.

In this model, `cargo build -p some_dep` would have the atomic bits "emit-bin", mark red all emit-exclusive queries, run those, and save them into the "emit-bin" atomic response (along with other emit-exclusive metadata)

Any activity looking for `some_dep`'s "emit-bin" information would request that, but if they do not request it, they'll be met with the information produced by the earlier cargo check.

So for the second cargo check, it would be like if `some_dep` was never built in the first place.

<!--
### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*
-->
