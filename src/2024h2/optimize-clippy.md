# Optimizing Clippy & linting
(a.k.a The Clippy Performance Project)

| Metadata |                                                              |
| -------- | ------------------------------------------------------------ |
| Owner(s) | [@blyxyas][blyxyas]                                          |
| Teams    | [Clippy team][clippy]                                                      |
| Status   | WIP                                                          |

## Summary

This is the formalization and documentation of the Clippy Performance Project, a project first talked about on [Zulip, July 2023](https://rust-lang.zulipchat.com/#narrow/stream/257328-clippy/topic/Clippy's.20performance). As the project consists of several points and is ever-changing, this document also has a dynamic structure and the team can add points. 

In short, this is an effort to optimize Clippy, and Rust's linting infrastructure with a point of view of making Clippy faster (both on CI/CD pipelines, and on devs' machines)

## Motivation

Clippy can take up to 2.5 times the time that a normal `cargo check` takes, and it doesn't need to be! Taking so long is expensive both in development time, and in real money.

On GitHub Actions, this excessive time can equal the cost of running `cargo check` on a Linux x64 32-cores machine, on a Linux x64 2-cores machine. (A 3.3-times increase).

### The status quo

The current status quo is that Clippy it's mostly a pre-commit / pre-merge tool. It isn't feasible to run it on-save or in the midst of working on a feature because it greatly stops the developer workflow, distracting the developer with a mindless waste of time, instead of working on the actual feature.

The other big use of Clippy is in CI, but the great slowness of the linting process often is a deterrent for using it as a workflow.

<!-- *Elaborate in more detail about the problem you are trying to solve. This section is making the case for why this particular problem is worth prioritizing with project bandwidth. A strong status quo section will (a) identify the target audience and (b) give specifics about the problems they are facing today. Sometimes it may be useful to start sketching out how you think those problems will be addressed by your change, as well, though it's not necessary.* -->

### The next 6 months

In order to achieve a better performance we want to:

- Keep working on, and eventually merge [rust#125116][pr125116]
- Improve of checking on proc-macros & expansions, maybe by precomputing expanded spans or memoizing the checking functions.
- Optimize checking for MSRVs and `#[clippy::msrv]` attributes. (Probably using static values, precomputing MSRV spans?)

Apart from these 3 clear goals, any open issue, open PR or merged PRs with the label [`performance-project`](https://github.com/rust-lang/rust-clippy/issues?q=sort%3Aupdated-desc+is%3Aopen+label%3Aperformance-project) are a great benefit.

### The "shiny future" we are working towards

**!! INITIAL TEXT, TEAM SHOULD REVIEW** 

The possible outcome would be a system that can be run on-save without being a hassle to the developer, and that has the minimum possible overhead over `cargo check` (which, would also be optimized as a side of a lot of a subset of the optimizations).

A developer shouldn't have to get a high-end machine to run a compiler swiftly; and a server should not spend more valuable seconds on linting than strictly necessary.

*This text is NORMATIVE, in the sense that teams should review this and make sure they are aligned. If not, then the shiny future should be moved to frequently asked questions with a title like "what might we do next".*

[da]: ../about/design_axioms.md

## Ownership

**Owner:** Alejandra Gonzalez, a.k.a. [@blyxyas][blyxyas]

<!-- I'm not sure if anyone else is interested in actively working on the CPP -->

<!-- Due to the dynamic nature of the project, I'm not sure what else to put here -->

### Frequently Asked Questions

<!-- #### How will improvements be measured? -->

[blyxyas]: https://github.com/blyxyas
[clippy]: https://github.com/rust-lang/rust-clippy
[pr125116]: https://github.com/rust-lang/rust/pull/125116