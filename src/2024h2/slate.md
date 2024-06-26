# The 2024H2 goal slate

This document explains the 2024H2 goal slate and how it was chosen. If you just want to see a table of goals, see the [all candidates](./candidates.md) page.

> *![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow) This document is a draft. The reasoning and [goal slate](./slate.md) are still evolving. If you have thoughts or suggestions, please reach out to nikomatsakis on the [#project-goals-2024h2](https://rust-lang.zulipchat.com/#narrow/stream/435869-project-goals-2024h2) Zulip stream.*

## Summary

This RFC presents the Rust project goal slate for 2024H2. The slate consists of NN total project goals of which we have selected three as our "flagship goals":

* Release the Rust 2024 edition (owner: TC)
* Bringing the Async Rust experience closer to parity with sync Rust (owners: tmandry, nikomatsakis)
* Resolving the biggest blockers to Linux building on stable Rust (owners: joshtriplett, nikomatsakis)

Flagship goals represent the goals expected to have the broadest overall impact. 

## Motivation

This RFC marks the first goal slate proposed under the experimental new roadmap process described in [RFC #3614][]. It consists of NN project goals, of which we have selected three as **flagship goals**. Flagship goals represent the goals expected to have the broadest overall impact. 

[RFC #3614]: https://github.com/rust-lang/rfcs/pull/3614

### How the goal process works

**Project goals** are proposed bottom-up by an **owner**, somebody who is willing to commit resources (time, money, leadership) to seeing the work get done. The owner identifies the problem they want to address and sketches the solution of how they want to do so. They also identify the support they will need from the Rust teams (typically things like review bandwidth or feedback on RFCs). Teams then read the goals and provide feedback. If the goal is approved, teams are committing to support the owner in their work. 

Project goals can vary in scope from an internal refactoring that affects only one team to a larger cross-cutting initiative. No matter its scope, accepting a goal should never be interpreted as a promise that the team will make any future decision (e.g., accepting an RFC that has yet to be written). Rather, it is a promise that the team are aligned on the contents of the goal thus far (including the design axioms and other notes) and will prioritize giving feedback and support as needed.

Of the proposed goals, a small subset are selected by the roadmap owner as **flagship goals**. Flagship goals are chosen for their high impact (many Rust users will be impacted) and their shovel-ready nature (the org is well-aligned around a concrete plan). Flagship goals are the ones that will feature most prominently in our public messaging and which should be prioritized by Rust teams where needed.

### Rust’s mission

Our goals are selected to further Rust's mission of **empowering everyone to build reliable and efficient software**. Rust targets programs that prioritize

* reliability and robustness;
* performance, memory usage, and resource consumption; and
* long-term maintenance and extensibility.

We consider "any two out of the three" to the right heuristic for projects where Rust is a strong contender or possibly the best option.

### Axioms for selecting goals

We believe that...

* **Rust must deliver on its promise of peak performance and high reliability.** Rust’s maximum advantage is in applications that require peak performance or low-level systems capabilities. We must continue to innovate and support those areas above all.
* **Rust's goals require high productivity and ergonomics.** Being attentive to ergonomics broadens Rust impact by making it more appealing for projects that value reliability and maintenance but which don't have strict performance requirements.
* **Slow and steady wins the race.** For this first round of goals, we want a small set that can be completed without undue stress. As the Rust open source org continues to grow, the set of goals can grow in size.

## Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

### Flagship goals

The flagship goals proposed for this roadmap are as follows:

* [**Release the Rust 2024 edition**](./Rust-2024-Edition.md), which will contain
    * a change in how `impl Trait` capture bounds work ([RFC #3498](https://github.com/rust-lang/rfcs/pull/3498) and [RFC #3617](https://github.com/rust-lang/rfcs/pull/3617))
    * reserving the `gen` keyword to allow for generators ([RFC #3513](https://github.com/rust-lang/rfcs/pull/3513))
    * never type fallback ([#123748](https://github.com/rust-lang/rust/issues/123748))
    * and a [number of other potential changes](https://github.com/rust-lang/rust/issues?q=label%3AC-tracking-issue+label%3AA-edition-2024+label%3AS-tracking-ready-to-stabilize%2CS-tracking-needs-documentation+-label%3AS-tracking-impl-incomplete%2CS-tracking-design-concerns) that may be included if they make enough progress
* [**Bringing the Async Rust experience closer to parity with sync Rust**](./async.md) via:
    * resolving the "send bound problem", thus enabling foundational, generic traits like Tower's [`Service`]() trait;
    * stabilizing async closures, thus enabling richer, combinator APIs like sync Rust's [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html);
    * reorganizing the async WG, so the project can benefit from a group of async rust experts with deep knowledge of the space that can align around a shared vision.
* [**Resolving the biggest blockers to Linux building on stable Rust**](./rfl_stable.md) via:
    * stabilizing support for arbitrary `self` types and unsizeable smart pointers, thus permitting ergonomic support for [in-place linked lists](https://rust-for-linux.com/arc-in-the-linux-kernel) on stable;
    * stabilizing features for labeled goto in inline assembler and extended `offset_of!` support, needed for various bts of low-level coding;
    * adding Rust For Linux project on Rust CI, thus ensuring we don't accidentally cause regressions for this highly visible project (done!);
    * stabilizing support for pointers to statics in constants, permitting the construction of vtables for kernel modules;

[MCP 727]: https://github.com/rust-lang/compiler-team/issues/727

#### Why these particular flagship goals?

**2024 Edition.** 2024 will mark the 4th Rust edition, following on the 2015, 2018, and 2021 editions. Similar to the [2021 edition](https://github.com/nikomatsakis/rfcs/blob/rfl-project-goal/text/3085-edition-2021.md), the 2024 edition is not a "major marketing push" but rather an opportunity to correct small ergonomic issues with Rust that will make it overall much easier to use. The changes planned for the 2024 edition will (1) support `-> impl Trait` and `async fn` in traits by aligning capture behavior; (2) permit (async) generators to be added in the future by reserving the `gen` keyword; and (3) alter fallback for the `!` type.

**Async.** In 2024 we plan to deliver several critical async Rust building block features, most notably support for *async closures* and *`Send` bounds*. This is part of a multi-year program aiming to raise the experience of authoring "async Rust" to the same level of quality as "sync Rust". Async Rust is a crucial growth area, with 52% of the respondents in the [2023 Rust survey](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html) indicating that they use Rust to build server-side or backend applications. 

**Rust for Linux.** The [experimental support for Rust development in the Linux kernel][RFL.com] is a watershed moment for Rust, demonstrating to the world that Rust is indeed capable of targeting all manner of low-level systems applications. And yet today that support rests on a [number of unstable features][RFL#2], blocking the effort from ever going beyond experimental status. For 2024H2 we will work to close the largest gaps that block support.

[RFL.com]: https://rust-for-linux.com/
[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2


### Project goals

The slate of project goals is still being selected. For the list of project goals under considation, see the [candidates page](./candidates.md#project-goals-under-consideration). These goals range from internal refactorings to important end-user features. What they have in common is that they have the backing of the Rust team(s) that own the areas they impact.

**Resourcing and plan.** Each goal requires an *overall owner* responsible for its progress as well as a **plan**, a fairly detailed list of the "tasks to be done" over the next six months along with the people responsible for those items (these may or may not be the owner).

**Orphaned goals.** In some cases there are elements of the plan that are "orphaned", meaning that there is no person who has has the time/resources/interest in doing them. Accepting these goals is a way for the Rust project to signal that this is work we would like to see happen and thus to encourage people to show up to do it. The Rust Foundation and other sponsors may also use these goals as a component in deciding when to offer grants or financial support.

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

The following table highlights the asks from each affected team.
The "owner" in the column is the person expecting to do the design/implementation work that the team will be approving.

| Team        | Goal                           | Owner                 | Ask                                                                         |
| ----------- | ------------------------------ | --------------------- | --------------------------------------------------------------------------- |
| ![Lang]     | [Async][]                      | nikomatsakis          | Approve RTN RFC and stabilization                                           |
|             | [Async][]                      | nikomatsakis, tmandry | Approve new team structure to replace async-wg                              |
|             | [Async][]                      | TBD                   | Approve async closure RFC and stabilization                                 |
|             | [Async][]                      | eholk                 | Provide feedback on async iteration RFC (stretch goal)                      |
|             | [Async][]                      | n/a                   | An estimated 4–5 design meetings                                            |
|             | [RFL][]                        | Adrian Taylor         | Stabilization decision for arbitrary self types v2                          |
|             | [RFL][]                        | Alice Ryhl            | Approve [RFC #3621][] and stabilize implementation                          |
|             | [RFL][]                        | Gary Guo              | Stabilize `asm_goto`                                                        |
|             | [Cargo Script][CS]             | Ed Page               | Stabilize cargo script backtick syntax                                      |
|             | [ATPIT][]                      | Oli Scherer           | Stabilize ATPIT                                                             |
|             | [Patterns of empty types][PET] | Nadrieril             | Review of "never patterns" RFC and stabilization decision                   |
|             | [Ergonomic Ref Counting][ERC]  | Jonathan Kelley       | Primary review and acceptance of "never patterns" RFC                       |
| ![Libs]     | [Async][]                      | nikomatsakis, tmandry | Approve new team structure to replace async-wg                              |
| ![Libs-API] | [Async][]                      | eholk                 | Approve async iteration RFC and stabilization (stretch goal)                |
|             | [RFL][]                        | Ding Xiang Fei        | Stabilize `offset_of!` syntax for struct fields                             |
|             | [RFL][]                        | Ding Xiang Fei        | Stabilize `offset_of!` syntax for struct fields                             |
|             | [Ergonomic Ref Counting][ERC]  | [Jonathan Kelley]     | Secondary review of Ergonomic Ref Counting RFC                              |
| ![Compiler] | [RFL][]                        | Adrian Taylor         | Review support and guidance for impl of arbitrary self types v2             |
|             | [RFL][]                        | Jakub Beránek         | Review and approve guidelines for RFL in CI                                 |
| ![Types]    | [Next-gen Solver][NGS]         | lcnr                  | stabilize the use of the next-generation trait solver in coherence checking |
|             | [ATPIT][]                      | Oli Scherer           | Stabilize ATPIT                                                             |
|             | [AMF][AMF]                     | nikomatsakis          | Participaton from 2 types team members in a-mir-formality                   |
|             | [Polonius][NBNLB]              | lqd                   | Review and support                                                          |

[RFC #3621]: https://github.com/rust-lang/rfcs/pull/3621

[Lang]: https://img.shields.io/badge/Lang-red
[Libs]: https://img.shields.io/badge/Libs-red
[Libs-API]: https://img.shields.io/badge/Libs--api-red
[Compiler]: https://img.shields.io/badge/Compiler-red
[Types]: https://img.shields.io/badge/Types-red

[RFL]: ./rfl_stable.md
[CS]: ./cargo-script.md
[Async]: ./async.md
[R]: ./rfl_stable.md
[ATPIT]: ./ATPIT.md
[NGS]: ./next-solver.md
[AMF]: ./a-mir-formality.md
[NBNLB]: ./Polonius.md
[PET]: ./Patterns-of-empty-types.md
[ERC]: ./ergonomic-rc.md