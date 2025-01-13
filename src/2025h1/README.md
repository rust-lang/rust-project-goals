> *![Status: Accepting goal proposals](https://img.shields.io/badge/Status-Accepting%20goal%20proposals-yellow) We are in the process of assembling the goal slate.*

## Summary

This is a draft for the eventual RFC proposing the 2025H1 goals.

## Motivation

The 2025H1 goal slate consists of <!-- #GOALS --> project goals, of which we have selected (TBD) as **flagship goals**. Flagship goals represent the goals expected to have the broadest overall impact.

### How the goal process works

**Project goals** are proposed bottom-up by a **point of contact**, somebody who is willing to commit resources (time, money, leadership) to seeing the work get done. The owner identifies the problem they want to address and sketches the solution of how they want to do so. They also identify the support they will need from the Rust teams (typically things like review bandwidth or feedback on RFCs). Teams then read the goals and provide feedback. If the goal is approved, teams are committing to support the owner in their work.

Project goals can vary in scope from an internal refactoring that affects only one team to a larger cross-cutting initiative. No matter its scope, accepting a goal should never be interpreted as a promise that the team will make any future decision (e.g., accepting an RFC that has yet to be written). Rather, it is a promise that the team are aligned on the contents of the goal thus far (including the design axioms and other notes) and will prioritize giving feedback and support as needed.

Of the proposed goals, a small subset are selected by the roadmap owner as **flagship goals**. Flagship goals are chosen for their high impact (many Rust users will be impacted) and their shovel-ready nature (the org is well-aligned around a concrete plan). Flagship goals are the ones that will feature most prominently in our public messaging and which should be prioritized by Rust teams where needed.

### Rust’s mission

Our goals are selected to further Rust's mission of **empowering everyone to build reliable and efficient software**. Rust targets programs that prioritize

* reliability and robustness;
* performance, memory usage, and resource consumption; and
* long-term maintenance and extensibility.

We consider "any two out of the three" as the right heuristic for projects where Rust is a strong contender or possibly the best option.

### Axioms for selecting goals

We believe that...

* **Rust must deliver on its promise of peak performance and high reliability.** Rust’s maximum advantage is in applications that require peak performance or low-level systems capabilities. We must continue to innovate and support those areas above all.
* **Rust's goals require high productivity and ergonomics.** Being attentive to ergonomics broadens Rust impact by making it more appealing for projects that value reliability and maintenance but which don't have strict performance requirements.
* **Slow and steady wins the race.** For this first round of goals, we want a small set that can be completed without undue stress. As the Rust open source org continues to grow, the set of goals can grow in size.

## Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

### Flagship goals

The flagship goals proposed for this roadmap are as follows:

* Continue making Rust easier to use for network systems by [**bringing the Async Rust experience closer to parity with sync Rust**](./async.md). In 2025H1 we plan to:
    * tell a complete story for the use of async fn in traits, unblocking wide ecosystem adoption;
    * improve the ergonomics of `Pin`, which is frequently used in low-level async code; and
    * prepare to support asynchronous (and synchronous) generators in the language.
* Continue helping Rust support low-level projects by [**stabilizing compiler options and tooling used by the Rust-for-Linux (RFL) project**](./rfl.md). In 2025H1 we plan to:
    * implement [RFC #3716] to allow stabilizing ABI-modifying compiler flags to control code generation, sanitizer integration, and so forth;
    * taking the first step towards stabilizing [`build-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) by [creating a stable way to rebuild core with specific compiler options](./build-std.md);
    * add rustdoc features to extract and customize rustdoc tests (`--extract-doctests`);
    * stabilize clippy configuration like `.clippy.toml` and `CLIPPY_CONF_DIR`;
    * stabilize compiler flags to extract dependency info (e.g., as via `-Zbinary-dep-depinfo=y`) and to configure no-std without requiring it in the source file (e.g., as via `-Zcrate-attr`);
* Address the biggest concerns raised by Rust maintainers, lack of face-to-face interaction, by [**organizing the Rust All-Hands 2025**](./all-hands.md). In 2025H1 we plan to:
    * convene Rust maintainers to celebrate Rust's tenth birthday at [RustWeek 2025](https://2025.rustweek.org) (co-organized with [RustNL](https://2025.rustweek.org/about/);
    * author a first draft for a [Rust vision doc](./rust-vision-doc.md) and gather feedback.

#### Why these particular flagship goals?

[**Async.**](./async.md) Rust is a great fit for server development thanks to its ability to scale to very high load while retaining low memory usage and tight tail latency. 52% of the respondents in the [2023 Rust survey](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html) indicated that they use Rust to build server-side or backend applications. In [2025H1 our plan](./async.md) is to deliver (a) improved support for async-fn-in-traits, completely subsuming the functionality of the [`async-trait` crate](https://crates.io/crates/async-trait); (b) finalize a design for sync and async generators, simplifying the creation of iterators and async data streams; (c) and improve the ergonomics of `Pin`, making lower-level async coding more approachable. These items together start to unblock the creation of the next generation of async libraries in the wider ecosystem, as progress there has been blocked on a stable solution for async traits and streams.

[**Rust for Linux.**](./rfl.md) The [experimental support for Rust development in the Linux kernel][RFL.com] is a watershed moment for Rust, demonstrating to the world that Rust is indeed a true alternative to C. Currently the Linux kernel support depends on a wide variety of unstable features in Rust; these same features block other embedded and low-level systems applications. We are working to stabilize all of these features so that RFL can be built on a stable toolchain. As we have successfully stabilized the majority of the language features used by RFL, we plan in 2025H1 to turn our focus to compiler flags and tooling options. We will (a) implement [RFC #3716] which lays out a design for ABI-modifying flags; (b) take the first step towards stabilizing [`build-std`](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std) by [creating a stable way to rebuild core with specific compiler options](./build-std.md); (c) extending rustdoc, clippy, and the compiler with features that extract metadata for integration into other build systems (in this case, the kernel's build system).

[**Rust All Hands 2025.**](./all-hands.md) May 15, 2025 marks the 10-year anniversary of Rust's 1.0 release; it also marks 10 years since the [creation of the Rust subteams](https://internals.rust-lang.org/t/announcing-the-subteams/2042). At the time [there were 6 Rust teams with 24 people in total](http://web.archive.org/web/20150517235608/http://www.rust-lang.org/team.html). There are now 57 teams with 166 people. In-person All Hands meetings are an effective way to help these maintainers get to know one another with high-bandwidth discussions. This year, the Rust project will be coming together for [RustWeek 2025](https://2025.rustweek.org), a joint event organized with [RustNL](https://2025.rustweek.org/about/). Participating project teams will use the time to share knowledge, make plans, or just get to know one another better. One particular goal for the All Hands is reviewing a draft of the [Rust Vision Doc](./rust-vision-doc.md), a document that aims to take stock of where Rust is and lay out high-level goals for the next few years.

[RFL.com]: https://rust-for-linux.com/
[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2

### Project goals

The full slate of project goals are as follows. These goals all have identified owners who will drive the work forward as well as a viable work plan. The goals include asks from the listed Rust teams, which are cataloged in the [reference-level explanation](#reference-level-explanation) section below.

**Invited goals.** Some goals of the goals below are "invited goals", meaning that for that goal to happen we need someone to step up and serve as an owner. To find the invited goals, look for the ![Help wanted][] badge in the table below. Invited goals have reserved capacity for teams and a mentor, so if you are someone looking to help Rust progress, they are a great way to get involved.

<!-- GOALS -->

# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

The following table highlights the asks from each affected team.
The "owner" in the column is the person expecting to do the design/implementation work that the team will be approving.

<!-- TEAM ASKS -->

### Definitions

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

# Frequently asked questions

## What goals were not accepted?

The following goals were proposed but not accepted:

<!-- GOALS NOT ACCEPTED -->

<!-- Github usernames -->
