- Feature Name: N/A
- Start Date: (fill me in with today's date, YYYY-MM-DD)
- RFC PR: [rust-lang/rfcs#373](https://github.com/rust-lang/rfcs/pull/373)
- Rust Issue: N/A

# Summary
[summary]: #summary

Propose a slate of (((#GOALS))) goals for 2025H2.

# Motivation

The 2025h2 goal slate consists of (((#GOALS))) project goals, of which we have selected a subset as **flagship goals**. Flagship goals represent the highest priority being done by the various Rust teams.


# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

## Rust's mission

Our goals are selected to further Rust's mission of making it dramatically more accessible to author and maintain *foundational software*—the software that underlies everything else. This includes the CLI tools and development infrastructure that developers rely on, the cloud platforms that run applications, the embedded systems in devices around us, and increasingly the kernels and operating systems that power it all.

Foundational software has particularly demanding requirements: reliability is paramount because when foundations fail, everything built on top fails too. Performance overhead must be minimized because it becomes a floor on what the layers above can achieve. Traditionally, meeting these requirements meant choosing between the power-but-danger of C/C++ or the safety-but-constraints of higher-level languages used in very specific ways.

Rust changes this balance by combining zero-cost abstractions with memory safety guarantees, often allowing you to write high-level code with low-level performance. While Rust's primary focus remains foundational software, we also recognize that supporting higher-level applications helps identify ergonomic improvements that benefit all users and enables developers to use Rust throughout their entire stack.

## Flagship goals

This period we have (((#FLAGSHIP GOALS))) flagship goals, broken out into four themes:

* [Beyond the `&`](#beyond-the-), making it possible to create user-defined smart pointers that are as ergonomic as Rust's built-in references `&`.
* [Unblocking dormant traits](#unblocking-dormant-traits), extending the core capabilities of Rust's trait system to unblock long-desired features for language interop, lending iteration, and more.
* [Flexible, fast(er) Rust builds](#flexible-faster-rust-builds), making Rust's builds fasterand improving support for specialized build scenarios like embedded usage and sanitizers.
* [Higher-level Rust](#higher-level-rust), making higher-level usage patterns in Rust easier.

### "Beyond the `&`"

(((FLAGSHIP GOALS: Beyond the `&`)))

One of Rust's core value propositions is that it's a "library-based language"—libraries can build abstractions that feel built-in to the language even when they're not. Smart pointer types like `Rc` and `Arc` are prime examples, implemented purely in the standard library yet feeling like native language features. However, Rust's built-in reference types (`&T` and `&mut T`) have special capabilities that user-defined smart pointers cannot replicate. This creates a "second-class citizen" problem where custom pointer types can't provide the same ergonomic experience as built-in references.

The "Beyond the `&`" initiative aims `&`'s special capabilities, allowing library authors to create smart pointers that are truly indistinguishable from built-in references in terms of syntax and ergonomics. This will enable more ergonomic smart pointers for use in cross-language interop (e.g., references to objects in other languages like C++ or Python) and for low-level projects like Rust for Linux which use smart pointers to express particular data structures.

### "Unblocking dormant traits"

(((FLAGSHIP GOALS: Unblocking dormant traits)))

Rust's trait system is one of its most powerful features, but it has a number of longstanding limitations that are preventing us from adopting new patterns. The goals in this category unblock a number of new capabilities:

* [Polonius](./polonius.md) will enable new borrowing patterns, and in particular [unblock "lending iterators"](https://github.com/rust-lang/rust/issues/92985). Over the last few goal periods we have identified an "alpha" vesion of polonius that addresses the most important cases while being relatively simple and optimizable. Our goal for 2025H2 is to implement this algorithm in a form that is ready for stabilization in 2026.
* The [next gen trait solver](./next-solver.md) is a refactored trait solver that unblocks better support for numerous language features (implied bounds, negative impls, the list goes on) in addition to closing a number of existing bugs and unsoundnesses. Over the last few goal periods, the trait solver went from early prototype to being production use in coherence. The goal for 2025H2 is to prepare it for use throughout the compiler.
* The work on [evolving trait hierarchies](./evolving-traits.md) will make it possible to refactor some parts of an existing trait out into a new supertrait so they can be used on their own. This unblocks a number of features where the existing trait is insufficiently general, in particular stabilizing support for custom receiver types, a prior project goal that wound up blocking on this refactoring.
* The work to [expand Rust's `Sized` hierarchy](./scalable-vectors.md) will permit us to express types that are neither `Sized` nor `?Sized`, such as extern types (which have no size) or ARM's Scalable Vector Extensions (which have a size that is known at runtime, but not compilation time). This goal builds on [RFC #3729][] and [RFC #3838][], authored in previous project goal periods.
* [In-place initialization](./in-place-initialization.md) allows creating structs and values that are tied to a particular place in memory. While useful directly for projects doing advanced C interop, it also unblocks expanding `dyn Trait` to support for `async fn` and `-> impl Trait` methods, as compiling such methods requires the ability for the callee to return a future whose size is not known to the caller.

### "Flexible, fast(er) Rust builds"

(((FLAGSHIP GOALS: Flexible, fast(er) Rust builds)))

The "Flexible, fast(er) Rust builds" initiative focuses on improving Rust's build system to better serve both specialized use cases and everyday development workflows:

* We are improving compilation performance through (1) [parallel compilation in the compiler front-end](./parallel-front-end.md), which delivers 20-30% faster builds, and (2) [making the Cranelift backend production-ready for development use](./production-ready-cranelift.md), offering roughly 20% faster code generation compared to LLVM for debug builds.
* We are working to [stabilize a core MVP of the `-Zbuild-std` feature](./build-std.md), which allows developers to rebuild the standard library from source with custom compiler flags. This unblocks critical use cases for embedded developers and low-level projects like Rust for Linux, while also enabling improvements like using sanitizers with the standard library or building `std` with debug information.

### "Higher-level Rust"

(((FLAGSHIP GOALS: Higher-level Rust)))

People generally start using Rust for foundational use cases, where the requirements for performance or reliability make it an obvious choice. But once they get used to it, they often find themselves turning to Rust even for higher-level use cases, like scripting, web services, or even GUI applications. Rust is often "surprisingly tolerable" for these high-level use cases -- except for some specific pain points that, while they impact everyone using Rust, hit these use cases particularly hard. We plan two flagship goals this period in this area:

* We aim to stabilize [cargo script](./cargo-script.md), a feature that allows single-file Rust programs that embed their dependencies, making it much easier to write small utilities, share code examples, and create reproducible bug reports without the overhead of full Cargo projects.
* We aim to finalize the design of [ergonomic ref-counting](./ergonomic-rc.md) and to finalize the experimental impl feature so it is ready for beta testing. Ergonomic ref counting makes it less cumbersome to work with ref-counted types like `Rc` and `Arc`, particularly in closures.

## Project goals

The full slate of project goals are as follows. These goals all have identified points of contact who will drive the work forward as well as a viable work plan. The goals include asks from the listed Rust teams, which are cataloged in the [reference-level explanation](#reference-level-explanation) section below.

**Invited goals.** Some goals of the goals below are "invited goals", meaning that for that goal to happen we need someone to step up and serve as a point of contact. To find the invited goals, look for the ![Help wanted][] badge in the table below. Invited goals have reserved capacity for teams and a mentor, so if you are someone looking to help Rust progress, they are a great way to get involved.

(((GOALS)))

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

## Goals broken out by champion

Who is championing which goals?

(((CHAMPIONS)))

## Team asks

The following table highlights the asks from each affected team.
The "owner" in the column is the person expecting to do the design/implementation work that the team will be approving.

(((TEAM ASKS)))

## Definitions

Definitions for terms used above:

* *Author RFC* and *Implementation* means actually writing the code, document, whatever.
* *Design meeting* means holding a synchronous meeting to review a proposal and provide feedback (no decision expected).
* *RFC decisions* means reviewing an RFC and deciding whether to accept.
* *Org decisions* means reaching a decision on an organizational or policy matter.
* *Secondary review* of an RFC means that the team is "tangentially" involved in the RFC and should be expected to briefly review.
* *Stabilizations* means reviewing a stabilization and report and deciding whether to stabilize.
* *Standard reviews* refers to reviews for PRs against the repository; these PRs are not expected to be unduly large or complicated.
* Other kinds of decisions:
    * [Lang team experiments](https://lang-team.rust-lang.org/how_to/experiment.html) are used to add nightly features that do not yet have an RFC. They are limited to trusted contributors and are used to resolve design details such that an RFC can be written.
    * Compiler [Major Change Proposal (MCP)](https://forge.rust-lang.org/compiler/mcp.html) is used to propose a 'larger than average' change and get feedback from the compiler team.
    * Library [API Change Proposal (ACP)](https://std-dev-guide.rust-lang.org/development/feature-lifecycle.html) describes a change to the standard library.

# Frequently asked questions

## How are project goals proposed?

**Project goals** are proposed bottom-up by a **point of contact**, somebody who is willing to commit resources (time, money, leadership) to seeing the work get done. The point of contact identifies the problem they want to address and sketches the solution of how they want to do so. They also identify the support they will need from the Rust teams (typically things like review bandwidth or feedback on RFCs). Teams then read the goals and provide feedback. If the goal is approved, teams are committing to support the point of contact in their work.

## What goals were not accepted?

The following goals were not accepted as nobody stepped up to champion them. This should not be taken as a rejection of the underlying idea but likely indicates bandwidth constraints or concerns about scope.

(((GOALS NOT ACCEPTED)))

## Does accepting a goal mean that the work is going to happen for sure?

No. Accepting a goal is not a promise to accept an RFC, stabilize a feature, or take any other binding action. Rather, it means that the team wants the goal to make progress and is committing to commit time to complete the Team Asks described in the goal. To give some concrete examples, when the compiler team accepts a goal, they are committing to make sure reviews get done, but they are not committing to give an `r+` if the code doesn't pass muster. Similarly, the lang team is agreeing to discuss an RFC and provide actionable feedback, but not necessarily to accept it.

## What is a "team champion"? What do they do?

Team champions are people who have volunteered to track progress on the goal and to serve as a liaison between the goal owner(s) and the team. They are committing to support the owner to avoid the goal getting stuck in some kind of procedural limbo. For example, the goal champion might make sure the goal gets discussed in a meeting, or help to find a reviewer for a PR that is stuck in the queue. (In cases where the goal owner is also on the team, they can serve as their own champion.)

# What do the column names like "Ded. r?" mean?

[valid_team_asks]: #what-do-the-column-names-like-ded-r-mean

Those column names refer to specific things that can be asked of teams:

(((VALID TEAM ASKS)))

## Do goals have to have champions to be accepted?

Yes -- to be accepted, a goal needs some champions. They don't necessarily have to have a champion for *every team*, particularly not those with minor asks, but they do need to have enough champions that it seems the goal owner will be adequately supported. Those champions also need to not be too overloaded.

## How will we avoid taking on too many goals?

That's a tough one. Part of the reason to have champions is to help us filter out goals -- if one champion has too many goals, or nobody is willing to champion the goal, that's a bad sign.

[AGS]: ./Project-goal-slate.md
[AMF]: ./a-mir-formality.md
[Async]: ./async.md
[ATPIT]: ./ATPIT.md
[CS]: ./cargo-script.md
[CT]: ./const-traits.md
[ERC]: ./ergonomic-rc.md
[MGCA]: ./min_generic_const_arguments.md
[NBNLB]: ./Polonius.md
[NGS]: ./next-solver.md
[PET]: ./Patterns-of-empty-types.md
[PGC]: ./pubgrub-in-cargo.md
[RFL]: ./rfl_stable.md
[SBS]: ./sandboxed-build-script.md
[YKR]: ./yank-crates-with-a-reason.md
[SC]: ./Rust-for-SciComp.md
[OC]: ./optimize-clippy.md

<!-- Github usernames -->