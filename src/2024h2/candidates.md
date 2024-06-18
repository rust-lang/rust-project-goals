# The 2024H2 goal candidates

This page lists all goals that have been proposed thus far. 

## Flagship vs team goals

Goal proposals come in two varieties

* **Flagship goals** are larger in scope and often cross-cutting. They typically have a number of "subgoals" representing individual deliverables. We pick a small number  of flagship goals each time, and truly completing a flagship goal will often take multiple goal periods.
* **Team goals** are smaller in scope and are often focused on a particular feature or other solution (as opposed to being a set of goals at a problem domain). They also include things like internal refactorings that most users wouldn't even be aware of.

Both kinds of goals use the same [template](../TEMPLATE.md), the difference is in the contents.

### Would you like to propose a goal?

Do it! [Instructions for proposing goals can be found here.](../how_to/propose_a_goal.md)*

## Flagship goals

### Accepted and proposed flagship goals

| 2024 goal                                                                 | Working towards             | Owner                              | RFC           |
| ------------------------------------------------------------------------- | --------------------------- | ---------------------------------- | ------------- |
| [Stabilize Rust 2024 edition][R2024]                                      |                             | [TC][]                             | [RFC #3501][] |
| [Bringing the Async Rust experience closer to parity with sync Rust][AFE] | Async/sync parity           | [nikomatsakis][], [tmandry][]      | [RFC #3657][] |
| [Resolving the biggest blockers to Linux building on stable Rust][LK]     | Linux builds on stable Rust | [nikomatsakis][], [joshtriplett][] | [RFC #3658][] |

[RFC #3657]: https://github.com/rust-lang/rfcs/pull/3657
[RFC #3658]: https://github.com/rust-lang/rfcs/pull/3658

### Flagship goals not accepted

The following goals were deemed premature to be considered flagship goals. However, parts of them are being proposed team-level goals, and they may be considered for flagship goals in the future.

| 2024 goal                                               | Working towards                       | Owner            | Teams              |
| ------------------------------------------------------- | ------------------------------------- | ---------------- | ------------------ |
| [Ergonomics initiative: clones and partial borrows][EI] | Entry-level Rust developer experience | [jkelleyrtp][]   | [Lang]             |
| [Faster iterative builds][FIB] ![WIP][wip]              | Entry-level Rust dev experience       | [jkelleyrtp][]   | [Lang], [Compiler] |
| [Rust for Scientific Computing][SCI] ![WIP][wip]        | Rust for Scientific Computing         | [ZuseZ4][]       | [Lang], [Compiler] |
| [Towards seamless C support][SCS] ![WIP][wip]           |                                       | [joshtriplett][] | [Lang], [Compiler] |

## Team goals

### Top candidates

| 2024 goal                                              | Accepted or proposed in | Owner            | Team    |
| ------------------------------------------------------ | ----------------------- | ---------------- | ------- |
| [Assemble goal slate][AGS]                             | [RFC #3614][]           | [nikomatsakis][] | [LC]    |
| [Cargo Script][CS]                                     | [#22][]                 | [epage]          | [Cargo] |
| [Next-generation trait solver][NTS]                    | (not yet accepted)      | [lcnr]           | [Types] |
| [Formal model of Rust][AMF]                            | (not yet accepted)      | [nikomatsakis]   | [Types] |
| [Polonius on Nightly][NBNLB]                           | (not yet accepted)      | [lqd]            | [Types] |
| [Stabilize Associated type positiom impl trait][ATPIT] | (not yet accepted)      | [oli-obk]        | [Types] |
| [Patterns of empty types][PET]                         | (not yet accepted)      | [Nadrieril]      | [Lang]  |         
| [Ergonomic ref-counting][RC]                           | (not yet accepted)      | [jkelleyrtp]     | [Lang]  |         

[#22]: https://github.com/rust-lang/rust-project-goals/issues/22
[RFC #3614]: https://github.com/rust-lang/rfcs/pull/3614
[RFC #3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html

## Other proposed goals

These are goals that are still being workshopped. They are sorted roughly by progress and likelihood to become top candidates.
In many cases the work being described will definitely happen, but it is not clear if they ought to become a "Project Goal".

| 2024 goal                        | Owner             | Teams              |
| -------------------------------- | ----------------- | ------------------ |
| [Contracts and invariants][CI]   | [pnkfelix]        | [Lang], [Compiler] |
| [Relaxing the Orphan Rule][RTOR] | [Josh-Triplett][] | [Lang]             |

[EI]: ./ergonomics-initiative.md
[FIB]: ./faster-iterative-builds.md
[AFE]: ./async.md
[LK]: ./rfl_stable.md
[LKH]: ./rfl_stable.md#ownership-and-other-resources
[SCS]: ./Seamless-C-Support.md
[CI]: ./Contracts-and-invariants.md
[NTS]: ./next-solver.md
[AMF]: ./a-mir-formality.md
[AGS]: ./Project-goal-slate.md
[R2024]: ./Rust-2024-Edition.md
[NBNLB]: ./Polonius.md
[PET]: ./Patterns-of-empty-types.md
[RTOR]: ./Relaxing-the-Orphan-Rule.md
[ATPIT]: ./Impl-trait-everywhere.md
[HLR]: ./higher-level-rust.md
[SCI]: ./Rust-for-SciComp.md
[CS]: ./cargo-script.md
[RC]: ./ergonomic-rc.md

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
[Cargo]: https://www.rust-lang.org/governance/teams/dev-tools#team-cargo
[Types]: https://www.rust-lang.org/governance/teams/compiler#team-types

[compiler-errors]: https://github.com/compiler-errors
[lcnr]: https://github.com/lcnr
[lqd]: https://github.com/lqd
[Nadrieril]: https://github.com/Nadrieril
[oli-obk]: https://github.com/oli-obk
[nikomatsakis]: https://github.com/nikomatsakis
[tmandry]: https://github.com/tmandry
[petrochenkov]: https://github.com/petrochenkov
[pnkfelix]: https://github.com/pnkfelix
[TC]: https://github.com/traviscross
[joshtriplett]: https://github.com/joshtriplett
[jkelleyrtp]: https://github.com/jkelleyrtp
[ZuseZ4]: https://github.com/ZuseZ4
[epage]: https://github.com/epage

[Help wanted]: https://img.shields.io/badge/Help%20wanted-blue
