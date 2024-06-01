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

### Accepted flagship goals

| 2024 goal                            | Working towards | Owner  | Accepted in   |
| ------------------------------------ | --------------- | ------ | ------------- |
| [Stabilize Rust 2024 edition][R2024] |                 | [TC][] | [RFC #3501][] |

### Candidate flagship goals

The following goals are nearing completion and expected to be converted into RFCs.

| 2024 goal                                                         | Working towards                       | Owner                               | Teams                       |
| ----------------------------------------------------------------- | ------------------------------------- | ----------------------------------- | --------------------------- |
| [Async closures and send bounds][AFE]                             | Async/sync parity                     | [nikomatsakis][], [tmandry][]       | [Lang], [Libs-API]          |
| [Stabilize key RFL features, RFL on CI][LK] [![Help wanted]][LKH] | Linux builds on stable Rust           | [nikomatsakis][], [joshtriplett][]  | [Lang], [Libs-API], [Infra] |


### WIP flagship goals

The following goals were submitted and are sitll "under revision". They are unlikely to become flagship goals this round, but some parts of them may be adopted as team goals.

| 2024 goal                                                         | Working towards                       | Owner                               | Teams                       |
| ----------------------------------------------------------------- | ------------------------------------- | ----------------------------------- | --------------------------- |
| [Ergonomics initiative: clones and partial borrows][EI]           | Entry-level Rust developer experience | [jkelleyrtp][]                      | [Lang]                      |
| [Faster iterative builds][FIB] ![WIP][wip]                        | Entry-level Rust dev experience       | [jkelleyrtp][]                      | [Lang], [Compiler]          |
| [Rust for Scientific Computing][SCI] ![WIP][wip]                  | Rust for Scientific Computing         | [ZuseZ4][]                          | [Lang], [Compiler]          |
| [Towards seamless C support][SCS] ![WIP][wip]                     |                                       | [joshtriplett][]                    | [Lang], [Compiler]          |

Some notes to highlight:

* The Rust For Linux goal has some "unfunded" elements around stabilizing compiler flags and customized variants of the library API. These are likely out of scope for 2024H2 but if resourcing could be found would be great to tackle.

## Team goals

### Accepted team goals

| 2024 goal                  | Accepted in   | Owner            |
| -------------------------- | ------------- | ---------------- |
| [Assemble goal slate][AGS] | [RFC #3614][] | [nikomatsakis][] |

[RFC #3614]: https://github.com/rust-lang/rfcs/pull/3614
[RFC #3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html

## Candidate team goals

These are goals that are still being workshopped. They are sorted roughly by progress and likelihood to become top candidates.
In many cases the work being described will definitely happen, but it is not clear if they ought to become a "Project Goal".

| 2024 goal                        | Owner             | Teams              |
| -------------------------------- | ----------------- | ------------------ |
| [Contracts and invariants][CI]   | [pnkfelix]        | [Lang], [Compiler] |
| [New Rust trait solver][NTS]     | [lcnr]            | [Types]            |
| [Formal model of Rust][AMF]      | [nikomatsakis]    | [Types]            |
| [Polonius on Nightly][NBNLB]     | [lqd]             | [Lang], [Types]    |
| [impl trait everywhere][ITE]     | [oli-obk]         | [Lang], [Types]    |
| [Patterns of empty types][PET]   | [Nadrieril]       | [Lang], [Compiler] |
| [Relaxing the Orphan Rule][RTOR] | [joshtriplett][]  | [Lang]             |

[EI]: ./ergonomics-initiative.md
[FIB]: ./faster-iterative-builds.md
[AFE]: ./async.md
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
[SCI]: ./Rust-for-SciComp.md

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
[Types]: https://www.rust-lang.org/governance/teams/compiler#team-types

[compiler-errors]: https://github.com/compiler-errors
[lcnr]: https://github.com/lcnr
[lqd]: https://github.com/lqd
[Nadrieril]: https://github.com/Nadrieril
[nikomatsakis]: https://github.com/nikomatsakis
[oli-obk]: https://github.com/oli-obk
[tmandry]: https://github.com/tmandry
[petrochenkov]: https://github.com/petrochenkov
[pnkfelix]: https://github.com/pnkfelix
[TC]: https://github.com/traviscross
[joshtriplett]: https://github.com/joshtriplett
[jkelleyrtp]: https://github.com/jkelleyrtp
[ZuseZ4]: https://github.com/ZuseZ4

[Help wanted]: https://img.shields.io/badge/Help%20wanted-blue
