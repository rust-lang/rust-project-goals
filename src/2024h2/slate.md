# The 2024H2 goal slate

> *![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow) These goals are still in draft form. They do not represent consensus. Expect changes. The goal column should describe the specific things you aim to get done in 2024H2; the theme ties those into a larger theme (it's ok to put N/A). [Would you like to propose a goal?](../how_to/propose_a_goal.md)*

This page describes the draft roadmap for 2024H2 (second half of 2024). Through conversations with Rust teams and customers, we have identified three **flagship goals** that represent Rust's top priorities at the moment.

* **Async fn everywhere**: Extend Rust's async support to cover the remaining gaps between async and sync, creating a foundation that supports a richer, more reliable, and more interoperable async ecosystem.
* **Linux kernel builds on stable Rust**: Stabilize the key features needed to support safe safe kernel development, using the Rust for Linux project as a testbed.
* **Joyful, scalable Rust teams**: Help the Rust open source project continue to grow and be a rewarding experience.

Important as these three flagship goals are, they don't cover everything. We have therefore included a final section:

* **The other 90%**: Standalone goals that don't directly tie to the above initiatives. The name "the other 90%" is meant to recognize that, while creating a great experience requires big, flashy projects, it *also* requires a ton of other work that often goes unrecognized, ranging from moderation to polish to refactoring. The goals in this section are likely to have the largest impact on your daily usage of Rust.

## Async fn everywhere

Async Rust is a crucial growth area, with a full 52% of the respondents in the [2023 Rust survey](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html) indicating that they use Rust to build server-side or backend applications. Despite that success, async is *also* the most frequently reported challenge to learning Rust.

Our goals for 2024H2 are to stabilize async closures; solve the send bound problem; and stabilize a trait for async iteration (also called streams). These goals will help propel us towards our long-term vision that async Rust should feel the same as sync Rust, with access to the same range of language support (traits, closures, dynamic dispatch, destructors), library support (including a clear "getting started" experience), and documentation (coverage in the Rust book).

| Goal                                            | Status      | Owner               | Teams              |
| ----------------------------------------------- | ----------- | ------------------- | ------------------ |
| [Revise async vision doc][]                     | ![WIP][wip] | [tmandry][]         | [Lang], [Libs-API] |
| [Stablilize async closures][]                   | ![WIP][wip] | [compiler-errors][] | [Lang], [Libs-API] |
| [Stable soluton for the "Send bound problem"][] | ![WIP][wip] | [nikomatsakis][]    | [Lang], [Libs-API] |
| [Stable trait for async iteration][]            | ![WIP][wip] | [eholk][]           | [Lang], [Libs-API] |

[Revise async vision doc]: ./Async.md
[Stablilize async closures]: ./Async--AsyncClosures.md
[Stable soluton for the "Send bound problem"]: Async--SendBounds.md
[Stable trait for async iteration]: ./Async--Streams.md

## Linux Kernel builds on Stable Rust

The experimental support for Rust development in the Linux kernel is a watershed moment for Rust, demonstrating to the world that Rust is indeed capable of targeting all manner of low-level systems applications. And yet that support today rests on a number of unstable features, blocking the effort from ever going beyond experimental status.

Our goal for 2024H2 is to stabilize the [features required by the Rust for Linux project][rfl]. This works towards our long-term goal of seeing Rust usage throughout kernels and low-level system software.

| Goal                                              | Status      | Owner                | Teams   |
| ------------------------------------------------- | ----------- | -------------------- | ------- |
| [Arbitrary self types][]                          | ![WIP][wip] | [Adrian Taylor][]    | [Lang]  |
| [DeriveSmartPointer][]                                 | ![WIP][wip] | [Alice Ryhl][]       | [Lang]  |
| [Pointers to statics in constants][]              | ![WIP][wip] | ![Owner needed][own] |         |
| Asm goto (not written)                            | ![WIP][wip] | ![Owner needed][own] |         |
| Offset of (not written)                           | ![WIP][wip] | ![Owner needed][own] |         |
| [RFL on Rust CI][]                                | ![WIP][wip] | ![Owner needed][own] | [Infra] |
| [Code-generation features and compiler options][] | ![WIP][wip] | ![Owner needed][own] |         |
| [Compiling core/alloc][]                          | ![WIP][wip] | ![Owner needed][own] |         |

## Joyful, scalable Rust teams

Rust's success is the result of the collective effort of thousands of contributors over time as well as its over XXX [official, long-term maintainers][gov]. But as Rust usage grows, we have seen scaling limits to the current structure: companies and individuals alike report that it can be difficult to follow what the Rust project is doing or to figure out how to contribute and get involved; maintainers find it difficult to grow their teams, resulting in stress and burnout.

in 2024H2, we aim to improve the situation through a number of efforts, including the process that led to the authoring of this roadmap. Our long-term vision is that members of the Rust teams feel well staffed and well supported. The Rust project governance should operate with transparency, making the current focus clear and making it easy for outside groups to propose new, bold initiatives. 

| Goal                                | Status                      | Owner                | Teams   |
| ----------------------------------- | --------------------------- | -------------------- | ------- |
| [Assemble goal slate][]             | [![Accepted][acc]][rfc3614] | nikomatsakis         |         |
| Track feature stabilization         | ![WIP][wip]                 |                      |         |
| [Finer-grained infra permissions][] | ![WIP][wip]                 | ![Owner needed][own] | [Infra] |
| Host Rust contributor event         | ![WIP][wip]                 |                      |         |
| Staff team                          | ![WIP][wip]                 |                      |         |

[Assemble goal slate]: ./Project-goal-slate.md

## "The other 90%"

Creating a great experience requires a combination of major initiatives (like the previous goals) and continuous polish and incremental improvement. As the saying goes, "we did the first 90% of the work, now we just need to do the other 90%". This section exists to capture important goals taken by teams across Rust which don't directly feed in to one of the above initiatives. Goals in this section are no less important than those in other sections: if anything, they are likely to have a larger impact on your daily usage of Rust.

| Goal                            | Status           | Owner                | Teams              |
| ------------------------------- | ---------------- | -------------------- | ------------------ |
| [Stabilize Rust 2024 edition][] | ![Accepted][acc] | TC                   | [LC]               |
| [Relaxing the Orphan Rule][]    | ![WIP][wip]      | ![Owner needed][own] | [Lang]             |
| [Polonius on nightly][]         | ![WIP][wip]      | [lqd]                | [Lang], [Types]    |
| [Impl trait everywhere][]       | ![WIP][wip]      | [oli-obk]            | [Lang], [Types]    |
| [Seamless C Support][]          | ![WIP][wip]      | ![Owner needed][own] | [Lang]             |
| [Patterns of empty types][]     | ![WIP][wip]      | [Nadrieril]          | [Lang], [Compiler] |


[rfc3614]: https://github.com/rust-lang/rfcs/pull/3614

[Stabilize Rust 2024 edition]: ./Rust-2024-Edition.md
[Intrusive linked lists]: ./Intrusive-linked-lists.md
[Fallible allocation]: ./Fallible-allocation.md
[Impl trait everywhere]: ./Impl-trait-everywhere.md
[Intrusive linked lists]: ./Intrusive-linked-lists.md
[Patterns of empty types]: ./Patterns-of-empty-types.md
[Polonius on nightly]: ./Polonius.md
[Relaxing the Orphan Rule]: ./Relaxing-the-Orphan-Rule.md

[own]: https://img.shields.io/badge/Owned%20Needed-blue

[acc]: https://img.shields.io/badge/Accepted-green
[prov]: https://img.shields.io/badge/Provisional-yellow
[wip]: https://img.shields.io/badge/WIP-yellow

[compiler-errors]: https://github.com/compiler-errors
[lqd]: https://github.com/lqd
[Nadrieril]: https://github.com/Nadrieril
[nikomatsakis]: https://github.com/nikomatsakis
[oli-obk]: https://github.com/oli-obk
[tmandry]: https://github.com/tmandry

[Compiler]: https://www.rust-lang.org/governance/teams/compiler
[Lang]: https://www.rust-lang.org/governance/teams/lang
[LC]: https://www.rust-lang.org/governance/teams/leadership-council
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api
