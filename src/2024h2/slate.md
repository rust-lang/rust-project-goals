# The 2024H2 goal slate

> *![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow) These goals are still in draft form. They do not represent consensus. Expect changes. The goal column should describe the specific things you aim to get done in 2024H2; the theme ties those into a larger theme (it's ok to put N/A). [Would you like to propose a goal?](../how_to/propose_a_goal.md)*

The goals vary in scope and size. Some of them are prominent, flagship goals (such as [improved async support][AFE] or [stabilizing the features required by the linux kernel][LK]) that will take multiple goal periods to complete. Others are smaller, discrete items that can be completed in six months. The table below is sorted so that the more prominent goals are listed towards the front (and goals that have been approved are listed first of all).

Note that committing to a goal means that the teams support the next few steps and are aligned on the overall vision. It is not a committment to accept any particular RFC or design and it is also not a commitment to continue working on the goal once the current goal period ends. We may find that, after taking the next few steps, we are no longer aligned to this goal.

## Accepted goals

These are goals that have been formally accepted.

| 2024 goal                            | Working towards    | Accepted in   | Owner            |
| ------------------------------------ | ------------------ | ------------- | ---------------- |
| [Assemble goal slate][AGS]           | Smooth project ops | [RFC #3614][] | [nikomatsakis][] |
| [Stabilize Rust 2024 edition][R2024] | Smooth project ops | [RFC #3501][] | [TC][]           |

[RFC #3614]: https://github.com/rust-lang/rfcs/pull/3614
[RFC #3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html

## Top candidate goals

These are goals that the slate owner is strongly considering submitting as RFCs.

| 2024 goal                                                         | Long-term ambition                    | Owner                               | Teams                       |
| ----------------------------------------------------------------- | ------------------------------------- | ----------------------------------- | --------------------------- |
| [Async closures and send bounds][AFE]                             | Async/sync parity                     | [nikomatsakis][], [tmandry][]       | [Lang], [Libs-API]          |
| [Stabilize key RFL features, RFL on CI][LK] [![Help wanted]][LKH] | Linux builds on stable Rust           | [nikomatsakis][], [Josh-Triplett][] | [Lang], [Libs-API], [Infra] |
| [Ergonomics initiative: clones and partial borrows][EI]           | Entry-level Rust developer experience | [jkelleyrtp][]                      | [Lang]                      |

Some notes to highlight:

* The Rust For Linux goal has some "unfunded" elements around stabilizing compiler flags and customized variants of the library API. These are likely out of scope for 2024H2 but if resourcing could be found would be great to tackle.

## Goals under discussion

These are goals that are still being workshopped. They are sorted roughly by progress and likelihood to become top candidates.
In many cases the work being described will definitely happen, but it is not clear if they ought to become a "Project Goal".

| 2024 goal                              | Long-term ambition              | Status      | Owner             | Teams              |
| -------------------------------------- | ------------------------------- | ----------- | ----------------- | ------------------ |
| [Faster iterative builds][FIB]         | Entry-level Rust dev experience | ![WIP][wip] | [jkelleyrtp][]    | [Lang], [Compiler] |
| [Rust for Scientific Computing][SCI]         | Rust for Scientific Computing | ![WIP][wip] | [Manuel S. Drehwald][]    | [Lang], [Compiler] |
| [Towards seamless C support][SCS]      |                                 | ![WIP][wip] | [Josh-Triplett][] | [Lang], [Compiler] |
| [Towards contracts and invariants][CI] |                                 | ![WIP][wip] | [pnkfelix]        | [Lang], [Compiler] |
| [Towards new Rust trait solver][NTS]   |                                 | ![WIP][wip] | [lcnr]            | [Types]            |
| [Towards a formal model of Rust][AMF]  |                                 | ![WIP][wip] | [nikomatsakis]    | [Types]            |
| [Polonius on Nightly][NBNLB]           |                                 | ![WIP][wip] | [lqd]             | [Lang], [Types]    |
| [impl trait everywhere][ITE]           |                                 | ![WIP][wip] | [oli-obk]         | [Lang], [Types]    |
| [Patterns of empty types][PET]         |                                 | ![WIP][wip] | [Nadrieril]       | [Lang], [Compiler] |
| [Relaxing the Orphan Rule][RTOR]       |                                 | ![WIP][wip] | [Josh-Triplett][] | [Lang]             |

[EI]: ./ergonomics-initiative.md
[FIB]: ./faster-iterative-builds.md
[AFE]: ./async_fn_everywhere.md
[LK]: ./rfl_stable.md
[LKH]: ./rfl_stable.md#ownership-and-other-resources
[SCS]: ./Seamless-C-Support.md
[CI]: ./Contracts-and-invariants.md
[NTS]: ./New-trait-solver.md
[AMF]: ./a-mir-formality.md
[AGS]: ./Project-goal-slate.md
[R2024]: ./Rust-2024-Edition.md
[NBNLB]: ./Polonius.md
[PET]: ./Patterns-of-empty-types.md
[RTOR]: ./Relaxing-the-Orphan-Rule.md
[ITE]: ./Impl-trait-everywhere.md
[HLR]: ./higher-level-rust.md

[Intrusive linked lists]: ./Intrusive-linked-lists.md
[Fallible allocation]: ./Fallible-allocation.md
[Intrusive linked lists]: ./Intrusive-linked-lists.md

[own]: https://img.shields.io/badge/Owner%20Needed-blue

[acc]: https://img.shields.io/badge/Accepted-green
[prov]: https://img.shields.io/badge/Provisional-yellow
[wip]: https://img.shields.io/badge/WIP-yellow

[Compiler]: https://www.rust-lang.org/governance/teams/compiler
[Lang]: https://www.rust-lang.org/governance/teams/lang
[LC]: https://www.rust-lang.org/governance/teams/leadership-council
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api
[Infra]: https://www.rust-lang.org/governance/teams/infra

[compiler-errors]: https://github.com/compiler-errors
[lqd]: https://github.com/lqd
[Nadrieril]: https://github.com/Nadrieril
[nikomatsakis]: https://github.com/nikomatsakis
[oli-obk]: https://github.com/oli-obk
[tmandry]: https://github.com/tmandry
[petrochenkov]: https://github.com/petrochenkov
[pnkfelix]: https://github.com/pnkfelix
[TC]: https://github.com/TC
[josh-triplett]: https://github.com/Josh-Triplett
[jkelleyrtp]: https://github.com/jkelleyrtp

[Help wanted]: https://img.shields.io/badge/Help%20wanted-blue
