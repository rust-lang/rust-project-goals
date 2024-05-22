# The 2024H2 goal slate

> *![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow) These goals are still in draft form. They do not represent consensus. Expect changes. The goal column should describe the specific things you aim to get done in 2024H2; the theme ties those into a larger theme (it's ok to put N/A). [Would you like to propose a goal?](../how_to/propose_a_goal.md)*

This page describes the draft roadmap for 2024H2 (second half of 2024). Through conversations with Rust teams and customers, we have identified two **key flagship goals** that will help advance Rust in particular domains:

* **Language support for trait-based async APIs**: Extend Rust language support with the features blocking trait-based, combinator APIs like async iterators, most notably [async closures][AC], and produce a new revision of the Async Vision Doc laying out the plan looking forward.
* **Linux kernel builds on stable Rust**: Stabilize features that block the Rust For Linux project from building on stable Rust.

In addition, we've assembled a list of **cross-cutting Rust improvements** that will improve the experience of using Rust across the board in ways big and small. Some of these improvements are targeting the functioning of the Rust open-source project itself (assembling this list of goals is one).

## Language support for trait-based async APIs

Async Rust is a crucial growth area, with a full 52% of the respondents in the [2023 Rust survey](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html) indicating that they use Rust to build server-side or backend applications. Despite that success, async is *also* the most frequently reported challenge to learning Rust. 

Our goals for 2024H2 are to stabilize async closures; solve the send bound problem; and stabilize a trait for async iteration (also called streams). These goals will help propel us towards our long-term vision that async Rust should feel the same as sync Rust, with access to the same range of language support (traits, closures, dynamic dispatch, destructors), library support (including a clear "getting started" experience), and documentation (coverage in the Rust book).

| Goal                                                | Status      | Owner               | Teams              |
| --------------------------------------------------- | ----------- | ------------------- | ------------------ |
| [Revise async vision doc][AVD]                      | ![WIP][wip] | [tmandry][]         | [Lang], [Libs-API] |
| [Stablilize async closures][AC]                     | ![WIP][wip] | [compiler-errors][] | [Lang], [Libs-API] |
| [Stable solution for the "Send bound problem"][ASB] | ![WIP][wip] | [nikomatsakis][]    | [Lang], [Libs-API] |
| [Stable trait for async iteration][AI]              | ![WIP][wip] | [eholk][]           | [Lang], [Libs-API] |
| [Complete async drop experiments][AD]               | ![WIP][wip] | [petrochenkov][]    | [Compiler]         |

[AVD]: ./Async.md
[AC]: ./Async--AsyncClosures.md
[ASB]: Async--SendBounds.md
[AI]: ./Async--Streams.md
[AD]: ./Async--Drop.md

## Linux Kernel builds on Stable Rust

The experimental support for Rust development in the Linux kernel is a watershed moment for Rust, demonstrating to the world that Rust is indeed capable of targeting all manner of low-level systems applications. And yet that support today rests on a number of unstable features, blocking the effort from ever going beyond experimental status.

Our goal for 2024H2 is to stabilize the [features required by the Rust for Linux project][rfl]. This works towards our long-term goal of seeing Rust usage throughout kernels and low-level system software.

| Goal                                            | Status      | Owner                | Teams              |
| ----------------------------------------------- | ----------- | -------------------- | ------------------ |
| [Unsizing in RFL Arc][]                         | ![WIP][wip] | [Alice Ryhl][]       | [Lang]             |
| [Arbitrary self types][]                        | ![WIP][wip] | [Adrian Taylor][]    | [Compiler], [Libs] |
| *Owners needed*                                 |             |                      |                    |
| ↳ RFL on Rust CI                                | ![WIP][wip] | ![Owner needed][own] | [Infra]            |
| ↳ Pointers to statics in constants              | ![WIP][wip] | ![Owner needed][own] | [Lang]             |
| ↳ Code-generation features and compiler options | ![WIP][wip] | ![Owner needed][own] | [Compiler]         |
| ↳ Compiling core/alloc                          | ![WIP][wip] | ![Owner needed][own] |                    |

[Alice Ryhl]: https://github.com/Darksonn/

Other issues that may merit goals but would need owners:

* `asm_goto` [#119364][] : Implemented, needs testing
* `offset_of` [#120140][], [#120141][]: Implemented, needs stabilization report and final decision on syntax

[Unsizing in RFL Arc]: ./rfl_arc_unsizing.md
[Arbitrary self types]: ./arbitrary_self_types.md
[Adrian Taylor]: https://github.com/adetaylor
[#119364]: https://github.com/rust-lang/rust/issues/119364
[#120140]: https://github.com/rust-lang/rust/issues/120140
[#120141]: https://github.com/rust-lang/rust/issues/120141

## Cross-cutting Rust improvements

Many of the most impactful changes to Rust are not tied to a particular domain. This section collects goals of this kind: general-purpose extensions or improvements that impact every Rust user. These can be major changes or they can be addressing a common papercut. In some cases, the goals here are targeted at improving the functioning of the Rust open-source organization, making contributing to and maintaining Rust a more joyful and productive experience.

| Goal                                 | Status                      | Owner                | Teams              |
| ------------------------------------ | --------------------------- | -------------------- | ------------------ |
| [Assemble goal slate][]              | [![Accepted][acc]][rfc3614] | nikomatsakis         | [LC]               |
| [Stabilize Rust 2024 edition][]      | ![Accepted][acc]            | TC                   | [LC]               |
| [Polonius on nightly][]              | ![WIP][wip]                 | [lqd]                | [Lang], [Types]    |
| [Impl trait everywhere][]            | ![WIP][wip]                 | [oli-obk]            | [Lang], [Types]    |
| *Team decision needed*               |                             |                      |                    |
| ↳ [Patterns of empty types][]        | ![WIP][wip]                 | [Nadrieril]          | [Lang], [Compiler] |
| ↳ [Contracts and invariants][]        | ![WIP][wip]                 | [pnkfelix]          | [Lang], [Compiler] |
| *Owners and/or team decision needed* |                             |                      |                    |
| ↳ [Relaxing the Orphan Rule][]       | ![WIP][wip]                 | ![Owner needed][own] | [Lang]             |
| ↳ [Seamless C support][]             | ![WIP][wip]                 | ![Owner needed][own] | [Lang]             |
| ↳ Track feature stabilization        | ![WIP][wip]                 | ![Owner needed][own] |                    |
| ↳ Finer-grained infra permissions    | ![WIP][wip]                 | ![Owner needed][own] | [Infra]            |
| ↳ Host Rust contributor event        | ![WIP][wip]                 | ![Owner needed][own] |                    |

[Assemble goal slate]: ./Project-goal-slate.md
[rfc3614]: https://github.com/rust-lang/rfcs/pull/3614
[Contracts and invariants]: ./Contracts-and-invariants.md
[Stabilize Rust 2024 edition]: ./Rust-2024-Edition.md
[Intrusive linked lists]: ./Intrusive-linked-lists.md
[Fallible allocation]: ./Fallible-allocation.md
[Impl trait everywhere]: ./Impl-trait-everywhere.md
[Intrusive linked lists]: ./Intrusive-linked-lists.md
[Patterns of empty types]: ./Patterns-of-empty-types.md
[Polonius on nightly]: ./Polonius.md
[Relaxing the Orphan Rule]: ./Relaxing-the-Orphan-Rule.md
[Seamless C support]: ./Seamless-C-Support.md

[own]: https://img.shields.io/badge/Owner%20Needed-blue

[acc]: https://img.shields.io/badge/Accepted-green
[prov]: https://img.shields.io/badge/Provisional-yellow
[wip]: https://img.shields.io/badge/WIP-yellow

[compiler-errors]: https://github.com/compiler-errors
[lqd]: https://github.com/lqd
[Nadrieril]: https://github.com/Nadrieril
[nikomatsakis]: https://github.com/nikomatsakis
[oli-obk]: https://github.com/oli-obk
[tmandry]: https://github.com/tmandry
[petrochenkov]: https://github.com/petrochenkov
[pnkfelix]: https://github.com/pnkfelix

[Compiler]: https://www.rust-lang.org/governance/teams/compiler
[Lang]: https://www.rust-lang.org/governance/teams/lang
[LC]: https://www.rust-lang.org/governance/teams/leadership-council
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api
