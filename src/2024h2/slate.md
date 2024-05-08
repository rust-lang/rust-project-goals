# The 2024H2 goal slate

## Accepted goals

*These goals have been accepted by the relevant teams and have an assigned owner.*

| Goal                    | Theme         | Owner            | Accepted by...                                                  |
| ----------------------- | ------------- | ---------------- | --------------------------------------------------------------- |
| [Assemble goal slate][] | Project goals | [nikomatsakis][] | [LC] in [RFC 3614](https://github.com/rust-lang/rfcs/pull/3614) |

[nikomatsakis]: https://github.com/nikomatsakis/

## Provisional goals in need of owners

*The team would like to commit to the goal, but the goal lacks an owner (this may be because owner is seeking funding). Learn more about [provisional goals](../about/provisional_goals.md) here.*

None

## WIP goals

*These goals are still in draft form. They do not represent consensus. Expect changes. The goal column should describe the specific things you aim to get done in 2024H2; the theme ties those into a larger theme (it's ok to put N/A). [Would you like to propose a goal?](../how_to/propose_a_goal.md)*

| Goal                                | Owner                     | Teams              |
| ----------------------------------- | ------------------------- | ------------------ |
| [Stabilize Rust 2024 edition][]     | Rust 2024 edition         | [LC]               |
| [Standard abstractions for async][] | [nikomatsakis], [tmandry] | [Lang], [Libs-API] |
| ↳ [Async closures][]                | [compiler-errors]         |                    |
| ↳ [Send bound problem][]            |                           |                    |
| Low-level systems development       |                           |                    |
| ↳ [Intrusive linked lists][]        | ![Owner needed][own]      | [Lang]             |
| ↳ [Fallible allocation][]           | ![Owner needed][own]      |                    |
| [Polonius on nightly][]             | [lqd]                     | [Lang], [Types]    |
| [Impl trait everywhere][]           | [oli-obk]                 | [Lang], [Types]    |
| [Seamless C Support][]              | ![Owner needed][own]      | [Lang]             |
| [Relaxing the Orphan Rule][]        | ![Owner needed][own]      | [Lang]             |
| [Patterns of empty types][]         | [Nadrieril]               | [Lang], [Compiler] |

## Not accepted goals

*The team does not want to commit to this goal, either because it doesn't seem like sufficiently high priority or because they do not have sufficient confidence that the goal as framed will get done.*

None.

[Assemble goal slate]: ./Project-goal-slate.md
[Async closures]: ./Async--AsyncClosures.md
[Fallible allocation]: ./Fallible-allocation.md
[Impl trait everywhere]: ./Impl-trait-everywhere.md
[Intrusive linked lists]: ./Intrusive-linked-lists.md
[Patterns of empty types]: ./Patterns-of-empty-types.md
[Polonius on nightly]: ./Polonius.md
[Relaxing the Orphan Rule]: ./Relaxing-the-Orphan-Rule.md
[Seamless C Support]: ./Seamless-C-Support.md
[Send bound problem]: Async--SendBounds.md
[Stabilize Rust 2024 edition]: ./Rust-2024-Edition.md
[Standard abstractions for async]: ./Async.md

[own]: https://img.shields.io/badge/Owned%20Needed-blue

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
[Types]: https://www.rust-lang.org/governance/teams/compiler#team-types
