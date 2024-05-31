# The 2024H2 goal slate

> *![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow) These goals are still in draft form. They do not represent consensus. Expect changes. The goal column should describe the specific things you aim to get done in 2024H2; the theme ties those into a larger theme (it's ok to put N/A). [Would you like to propose a goal?](../how_to/propose_a_goal.md)*

The goals vary in scope and size. Some of them are prominent, flagship goals (such as [improved async support][AFE] or [stabilizing the features required by the linux kernel][LK]) that will take multiple goal periods to complete. Others are smaller, discrete items that can be completed in six months. The table below is sorted so that the more prominent goals are listed towards the front (and goals that have been approved are listed first of all).

Note that committing to a goal means that the teams support the next few steps and are aligned on the overall vision. It is not a committment to accept any particular RFC or design and it is also not a commitment to continue working on the goal once the current goal period ends. We may find that, after taking the next few steps, we are no longer aligned to this goal.

| Goal                                                                   | Status                      | Owner             | Teams                                   |
| ---------------------------------------------------------------------- | --------------------------- | ----------------- | --------------------------------------- |
| [Assemble goal slate][AGS]                                             | [![Accepted][acc]][rfc3614] | [nikomatsakis][]  | [LC]                                    |
| [Stabilize Rust 2024 edition][R2024]                                   | ![Accepted][acc]            | [TC][]            | [LC]                                    |
| [Async closures and send bounds][AFE]                                  | ![WIP][wip]                 | [tmandry][]       | [Lang], [Libs-API]                      |
| [Stabilize language features used by RFL][LK] [![Help wanted]][LKH] | ![WIP][wip]                 | [Josh-Triplett][] | [Lang], [Libs-API], [Compiler], [Infra] |
| [Towards seamless C support][SCS]                                      | ![WIP][wip]                 | [Josh-Triplett][] | [Lang], [Compiler]                      |
| [Towards contracts and invariants][CI]                                 | ![WIP][wip]                 | [pnkfelix]        | [Lang], [Compiler]                      |
| [Towards new Rust trait solver][NTS]                                   | ![WIP][wip]                 | [lcnr]            | [Types]                                 |
| [Towards a formal model of Rust][AMF]                                  | ![WIP][wip]                 | [nikomatsakis]    | [Types]                                 |
| [Polonius on Nightly][NBNLB]                                           | ![WIP][wip]                 | [lqd]             | [Lang], [Types]                         |
| [impl trait everywhere][ITE]                                           | ![WIP][wip]                 | [oli-obk]         | [Lang], [Types]                         |
| [Patterns of empty types][PET]                                         | ![WIP][wip]                 | [Nadrieril]       | [Lang], [Compiler]                      |
| [Relaxing the Orphan Rule][RTOR]                                       | ![WIP][wip]                 | [Josh-Triplett][] | [Lang]                                  |

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

[rfc3614]: https://github.com/rust-lang/rfcs/pull/3614
[Intrusive linked lists]: ./Intrusive-linked-lists.md
[Fallible allocation]: ./Fallible-allocation.md
[Intrusive linked lists]: ./Intrusive-linked-lists.md

[own]: https://img.shields.io/badge/Owned%20Needed-blue

[acc]: https://img.shields.io/badge/Accepted-green
[prov]: https://img.shields.io/badge/Provisional-yellow
[wip]: https://img.shields.io/badge/WIP-yellow

[Compiler]: https://www.rust-lang.org/governance/teams/compiler
[Lang]: https://www.rust-lang.org/governance/teams/lang
[LC]: https://www.rust-lang.org/governance/teams/leadership-council
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api


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

[Help wanted]: https://img.shields.io/badge/Help%20wanted-blue
