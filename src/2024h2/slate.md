# The 2024H2 goal slate

This document explains the 2024H2 goal slate and how it was chosen. If you just want to see a table of goals, see the [all candidates](./candidates.md) page.

> *![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow) This document is a draft. The reasoning and [goal slate](./slate.md) are still evolving. If you have thoughts or suggestions, please reach out to nikomatsakis on the [#project-goals-2024h2](https://rust-lang.zulipchat.com/#narrow/stream/435869-project-goals-2024h2) Zulip stream.*

## Rust’s mission

Rust's mission is to empower everyone to build reliable and efficient software.
Rust targets programs that prioritize

* reliability and robustness;
* performance, memory usage, and resource consumption; and
* long-term maintenance and extensibility.

We consider "any two out of the three" to the right heuristic for projects where Rust is a strong contender or possibly the best option.

## Axioms for selecting goals

We believe that...

* **Rust must deliver on its promise of peak performance and high reliability.** Rust’s maximum advantage is in applications that require peak performance or low-level systems capabilities. We must continue to innovate and support those areas above all.
* **Rust's goals require high productivity and ergonomics.** Being attentive to ergonomics broadens Rust impact by making it more appealing for projects that value reliability and maintenance but which don't have strict performance requirements.
* **Slow and steady wins the race.** For this first round of goals, we want a small set that can be completed without undue stress. As the Rust open source org continues to grow, the set of goals can grow in size.

## How the goal process works

Goals are proposed bottom-up by Rust users who are willing to commit resources to see them get done, either by serving as owner,
or by funding, mentoring, or otherwise helping to find an owner.
Each goal describes a **problem** to be solved along with a **rough sketch of the solution**.
Although goals are often part of a longer term ambition, they themselves cover about 6 months of effort.

To be accepted, each goal requires both an **owner** and **team approval**:

* The **owner** is an individual (or set of individuals) who have committed to devote time and resources to getting the work done.
  Requiring an owner helps to avoid overpromising, announcing goals that lack the resources to make progress.
* **Team approval** indicates that the team agrees the problem is real and that the proposed solution sounds right.
  It also means the team has budgeted time to support the owner in trying to solve it.
  Requiring team approval ensures that owners don't invest in work only to see it be rejected or simply languish without feedback.

The balance between owners and teams also helps to drive prioritization.
Having an owner is a signal of the goal's importance: it means that *somebody* cares enough to put forward time/money.
The team's job is to ensure that goals are congruent with Rust's mission overall as well as selecting goals with the broadest impact.

No matter how it is approved, taking on a goal never means the project **must** make a change.
Goals are taken before a solution is known, and it may be that an acceptable solution cannot be found.
Final decisions are made the same way they've ever been, with RFCs to spell out the design and stabilization to mark a complete implementation.

## Flagship goals

Flagship goals represent the most impactful and ambitious goals that we aim to achieve. They are often part of a larger program that will extend started before the current six month goal period and which will extend into the future. Because of their importance, flagship goals are accepted by RFC (as linked below).

[RFC #3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html
[RFC #3657]: https://github.com/rust-lang/rfcs/pull/3657
[RFC #3658]: https://github.com/rust-lang/rfcs/pull/3658

* [**Release the Rust 2024 edition**](./Rust-2024-Edition.md), accepted in [RFC #3501][], [will contain](./Rust-2024-Edition.md#the-next-few-steps)
    * a change in how `impl Trait` capture bounds work ([RFC #3498](https://github.com/rust-lang/rfcs/pull/3498) and [RFC #3617](https://github.com/rust-lang/rfcs/pull/3617))
    * reserving the `gen` keyword to allow for generators ([RFC #3513](https://github.com/rust-lang/rfcs/pull/3513))
    * never type fallback ([#123748](https://github.com/rust-lang/rust/issues/123748))
    * and a [number of other potential changes](https://github.com/rust-lang/rust/issues?q=label%3AC-tracking-issue+label%3AA-edition-2024+label%3AS-tracking-ready-to-stabilize%2CS-tracking-needs-documentation+-label%3AS-tracking-impl-incomplete%2CS-tracking-design-concerns) that may be included if they make enough progress
* [**Bringing the Async Rust experience closer to parity with sync Rust**](./async.md), proposed in [RFC #3657][], [via](./async.md#the-next-few-steps):
    * resolving the "send bound problem", thus enabling foundational, generic traits like Tower's [`Service`]() trait;
    * stabilizing async closures, thus enabling richer, combinator APIs like sync Rust's [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html);
    * reorganizing the async WG, so the project can benefit from a group of async rust experts with deep knowledge of the space that can align around a shared vision.
* [**Resolving the biggest blockers to Linux building on stable Rust**](./rfl_stable.md), proposed in [RFC #3658][], [via](./rfl_stable.md#the-next-few-steps):
    * stabilizing support for arbitrary `self` types and unsizeable smart pointers, thus permitting ergonomic support for [in-place linked lists](https://rust-for-linux.com/arc-in-the-linux-kernel) on stable;
    * stabilizing features for labeled goto in inline assembler and extended `offset_of!` support, needed for various bts of low-level coding;
    * adding Rust For Linux project on Rust CI, thus ensuring we don't accidentally cause regressions for this highly visible project (done!);
    * stabilizing support for pointers to statics in constants, permitting the construction of vtables for kernel modules;

[MCP 727]: https://github.com/rust-lang/compiler-team/issues/727

## Project goals

What follows are the additional project goals. These goals range from internal refactorings to important end-user features. What they have in common is that they have the backing of the Rust team(s) that own the areas they impact. Project goals are accepted by pinging the leads of the teams affected, who are expected to consult with their team. This lightweight process avoids undue overhead.

**Resourcing and plan.** Each goal requires an *overall owner* responsible for its progress as well as a **plan**, a fairly detailed list of the "tasks to be done" over the next six months along with the people responsible for those items (these may or may not be the owner).

**Orphaned goals.** In some cases there are elements of the plan that are "orphaned", meaning that there is no person who has has the time/resources/interest in doing them. Accepting these goals is a way for the Rust project to signal that this is work we would like to see happen and thus to encourage people to show up to do it. The Rust Foundation and other sponsors may also use these goals as a component in deciding when to offer grants or financial support.

**Goals under consideration:** For the list of project goals under considation, see the [candidates page](./candidates.md#project-goals-under-consideration).

[Owner Needed]: https://img.shields.io/badge/Owner%20Needed-blue
