# The 2024H2 goal candidates

This page lists all goals that have been proposed thus far. 

## Project goals

Project goals represent a specific goal that the Rust teams would like to make progress on. Each goal is proposed by an **owner**, someone who will take responsibility for moving it forward. The goal identifies the problem and provides a rough outline of the solution along with an action plan for the work to be done. Teams review the goal to see that they are aligned with the plan and priorities of the owner and they can provide the various asks that the owner has made (e.g., to review RFCs and provide prompt feedback). If they are aligned, they teams accept the goal, and it becomes official.

### Flagship goals

Of the project goals, a small number are selected as **flagship** goals. These are the most impactful and ambitious goals that we will be the focus of our public messaging. The goal owner puts extra "top-down" effort into helping to shape these goals into a workable plan and find resources to complete that plan.

Factors considered to determine flagship goals:

* **Impact:** which Rust users will be affected and how?
* **Shovel-ready:** do we have a fairly concrete idea how to proceed, or is there early research to be done figuring things out?
* **Unsurprising:** a good flagship goal represents an established consensus. It's not a good thing if we declare a flagship goal that takes the Rust community (especially the Rust teams!) completely by surprise.

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

[R2024]: ./Rust-2024-Edition.md
[LK]: ./rfl_stable.md
[AFE]: ./async.md

## Project goals under consideration

### Top candidates

| 2024 goal                                                    | Accepted or proposed in | Owner            | Team                 |
| ------------------------------------------------------------ | ----------------------- | ---------------- | -------------------- |
| [Assemble goal slate][AGS]                                   | [RFC #3614][]           | [nikomatsakis][] | [LC]                 |
| [Cargo Script][CS]                                           | [#22][]                 | [epage]          | [Cargo], [Lang]      |
| [Extend pubgrub to match cargo's dependency resolution][PGC] | (not yet accepted)      | Eh2406           | [Cargo][]            |
| [Next-generation trait solver][NTS]                          | (not yet accepted)      | [lcnr]           | [Types]              |
| [Testing infra + contributors for a-mir-formality][AMF]      | (not yet accepted)      | [nikomatsakis]   | [Types]              |
| [Scalable Polonius support on nightly][NBNLB]                | (not yet accepted)      | [lqd]            | [Types]              |
| [Stabilize Associated type position impl trait][ATPIT]       | (not yet accepted)      | [oli-obk]        | [Types], [Lang]      |
| [Min generics const argument][MGCA]                          | (not yet accepted)      | BoxyUwU          | [Types][]            |
| [Patterns of empty types][PET]                               | (not yet accepted)      | [Nadrieril]      | [Lang]               |
| [Ergonomic ref-counting][RC]                                 | (not yet accepted)      | [jkelleyrtp]     | [Lang], [Libs-API][] |
| [Const traits][CT]                                           | (not yet accepted)      | feel1-dead       | [Lang], [Libs-API][] |
| [Relaxing the Orphan Rule][RTOR]                             | (not yet accepted)      | ![Help wanted][] | [Lang], [Types][]    |

[AGS]: ./Project-goal-slate.md
[CS]: ./cargo-script.md
[NTS]: ./next-solver.md
[AMF]: ./a-mir-formality.md
[NBNLB]: ./Polonius.md
[ATPIT]: ./ATPIT.md
[PET]: ./Patterns-of-empty-types.md
[RC]: ./ergonomic-rc.md
[MGCA]: ./min_generic_const_arguments.md
[CT]: ./const-traits.md
[PGC]: ./pubgrub-in-cargo.md

[#22]: https://github.com/rust-lang/rust-project-goals/issues/22
[RFC #3614]: https://github.com/rust-lang/rfcs/pull/3614
[RFC #3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html

## Other proposed goals

These are goals that are still being workshopped. They are sorted roughly by progress and likelihood to become top candidates.
In many cases the work being described will definitely happen, but it is not clear if they ought to become a "Project Goal".

| 2024 goal                      | Owner      | Teams              |
| ------------------------------ | ---------- | ------------------ |
| [Contracts and invariants][CI] | [pnkfelix] | [Lang], [Compiler] |

[CI]: ./Contracts-and-invariants.md
[RTOR]: ./Relaxing-the-Orphan-Rule.md

## Goals not accepted

### Deferred flagship goals

The following goals were deemed to be too large in scope and insufficiently baked to be considered for flagship goals in this round (however noble their intent). In many cases we have identified smaller pieces of these goals and pulled them out as project goals. Looking forward we will continue iterating to determine if the goal can be used in a future round of goal planning.

| 2024 goal                                               | Working towards                       | Owner            | Teams              |
| ------------------------------------------------------- | ------------------------------------- | ---------------- | ------------------ |
| [Ergonomics initiative: clones and partial borrows][EI] | Entry-level Rust developer experience | [jkelleyrtp][]   | [Lang]             |
| [Faster iterative builds][FIB] ![WIP][wip]              | Entry-level Rust dev experience       | [jkelleyrtp][]   | [Lang], [Compiler] |
| [Rust for Scientific Computing][SCI] ![WIP][wip]        | Rust for Scientific Computing         | [ZuseZ4][]       | [Lang], [Compiler] |
| [Towards seamless C support][SCS] ![WIP][wip]           |                                       | [joshtriplett][] | [Lang], [Compiler] |

[EI]: ./ergonomics-initiative.md
[FIB]: ./faster-iterative-builds.md
[SCI]: ./Rust-for-SciComp.md
[SCS]: ./Seamless-C-Support.md

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
