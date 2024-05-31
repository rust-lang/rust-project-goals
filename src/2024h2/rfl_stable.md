# Stabilize key RFL features, RFL on CI

| Metadata |                                                  |
| -------- | ------------------------------------------------ |
| Owner(s) | [nikomatsakis][], [Josh-Triplett][]              |
| Teams    | [Lang], [Libs-API], [Infra] |
| Status   | WIP                                              |

## Motivation

The experimental support for Rust development in the Linux kernel is a watershed moment for Rust, demonstrating to the world that Rust is indeed capable of targeting all manner of low-level systems applications. And yet that support today rests on a number of unstable features, blocking the effort from ever going beyond experimental status. For 2024H2 we will work to close the largest gaps that block support.

[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2

### The status quo

The [Rust For Linux (RFL)][RFL] project has been accepted into the Linux kernel in experimental status.
RFL permits authoring Rust-based modules that are compiled and linked into the Linux kernel.
This is a very exciting milestone for Rust, but it's also a big challenge.

[RFL]: https://rust-for-linux.com/

Integrating Rust into the Linux kernel means that Rust must be able to interoperate with the kernel's low-level C primitives for things like locking, linked lists, allocation, and so forth.
Integration requires Rust to expose low-level capabilities that don't all have stable interfaces.
In some cases, the needed features may be stable, but may not be enough to provide for an ergonomic coding style (e.g., see the macros used for [pinned (in-place) initialization][pinned-init]).

[pinned-init]: https://rust-for-linux.com/pinned-init
[arclk]: https://rust-for-linux.com/arc-in-the-linux-kernel

In the short term, the biggest blocker to the RFL exiting "experimental" status is its use of unstable features.
Because unstable features have no kind of reliability guarantee, this in turn means that RFL can only be built with a specific, pinned version of the Rust compiler.
This is a challenge for distributions which wish to be able to build a range of kernel sources with the same compiler, rather than having to select a particular toolchain for a particular kernel version.

Longer term, having Rust in the Linux kernel is an opportunity to expose more C developers to the benefits of using Rust.
But that exposure can go both ways.
If Rust is constantly causing pain related to toolchain instability,
or if Rust isn't able to interact gracefully with the kernel's data structures,
kernel developers may have a bad first impression that causes them to write off Rust altogether.
We wish to avoid that outcome.
And besides, the Linux kernel is exactly the sort of low-level systems application we want Rust to be great for!

For deeper background, please refer to these presentations on Rust For Linux that give more information:

* [Rust in the linux kernel / RustLab 2023 / Alice Ryhl](https://www.youtube.com/watch?v=CEznkXjYFb4)

### The next few steps

The RFL project has a [tracking issue][rfl2] listing the unstable features that they rely upon.
After discussion with the RFL team, we identified the following subgoals as the ones most urgent to address in 2024.
Closing these issues gets us within striking distance of being able to build the RFL codebase on stable Rust.

* Stable support for RFL's customized ARC type
* Labeled goto in inline assembler and extended `offset_of!` support
* RFL on Rust CI
* Pointers to statics in constants [![Owner Needed][]](#ownership-and-other-resources)
* Custom builds of core/alloc with specialized configuration options [![Owner Needed][]](#ownership-and-other-resources)
* Code-generation features and compiler options [![Owner Needed][]](#ownership-and-other-resources)

#### Stable support for RFL's customized ARC type

One of Rust's great features is that it doesn't "bake in" the set of pointer types.
The common types users use every day, such as `Box`, `Rc`, and `Arc`, are all (in principle) library defined.
But in reality those types enjoy access to some unstable features that let them be used more widely and ergonomically.
Since few users wish to define their own smart pointer types, this is rarely an issue and there has been relative little pressure to stabilize those mechanisms.

The RFL project needs to integrate with the Kernel's existing reference counting types and intrusive linked lists.
To achieve these goals they've created their own variant of [`Arc`][arclk] (hereafter denoted as `rfl::Arc`),
but this type cannot be used as idiomatically as the `Arc` type found in `libstd` without two features:

* The ability to be used in methods (e.g., `self: rfl::Arc<Self>`), aka "arbitrary self types", specified in [RFC 3519][].
* The ability to be coerce to dyn types like `rfl::Arc<dyn Trait>` and then support invoking methods on `Trait` through dynamic dispatch.
    * This requires the use of two unstable traits, `CoerceUnsized` and `DynDispatch`, neither of which are close to stabilization.
    * However, [RFC 3621][] provides for a "shortcut" -- a stable interface using `derive` that expands to those traits, leaving room to evolve the underlying details.

Our goal for 2024 is to close those gaps, most likely by implementing and stabilizing [RFC 3519][] and [RFC 3621][].

[rfl2]: https://github.com/Rust-for-Linux/linux/issues/2
[RFC 3519]: https://github.com/rust-lang/rfcs/pull/3519
[RFC 3621]: https://github.com/rust-lang/rfcs/pull/3621

#### Labeled goto in inline assembler and extended `offset_of!` support

These are two smaller extensions required by the Rust-for-Linux kernel support.
Both have been implemented but more experience and/or may be needed before stabilization is accepted.

#### RFL on Rust CI

Rust sometimes integrate external projects of paticular importance or interest into its CI.
This gives us early notice when changes to the compiler or stdlib impact that project.
Some of that breakage is accidental, and CI integration ensures we can fix it without the project ever being impacted.
Otherwise the breakage is intentional, and this gives us an early way to notify the project so they can get ahead of it.

Because of the potential to slow velocity and incur extra work,
the bar for being integrated into CI is high, but we believe that Rust For Linux meets that bar.
Given that RFL would not be the first such project to be integrated into CI,
part of pursuing this goal should be establishing clearer policies on when and how we integrate external projects into our CI,
as we now have enough examples to generalize somewhat.

#### Pointers to statics in constants

The RFL project has a need to create vtables in read-only memory (unique address not required). The current implementation relies on the `const_mut_refs` and `const_refs_to_static` features ([representative example](https://godbolt.org/z/r58jP6YM4)).

### The "shiny future" we are working towards

The ultimate goal is to enable smooth and ergonomic interop between Rust and the Linux kernel's idiomatic data structures.

In addition to the work listed above, there are a few other obvious items that the Rust For Linux project needs. If we can find owners for these this year, we could even get them done as a "stretch goal":

* **Custom builds of core/alloc with specialized configuration options:** The RFL project builds the stdlib with a number of configuration options to eliminate undesired aspects of libcore (listed in [RFL#2][]). They need a standard way to build a custom version of core as well as agreement on the options that the kernel will continue using.
* **Code-generation features and compiler options:** The RFL project requires various code-generation options. Some of these are related to custom features of the kernel, such as [X18 support][#748] but others are codegen options like sanitizers and the like. Some subset of the options listed on [RFL#2][] will need to be stabilized to support being built with all required configurations, but working out the precise set will require more effort.

Looking further afield, possible future work includes more ergonomic versions of the [special patterns for safe pinned initialization](https://rust-for-linux.com/the-safe-pinned-initialization-problem) or a solution to [custom field projection for pinned types or other smart pointers](https://github.com/rust-lang/rfcs/pull/3318).

## Design axioms

None authored.

## Ownership and other resources

Here is a detailed list of the work to be done and who is expected to do it. This table includes the work to be done by owners and the work to be done by Rust teams (subject to approval by the team in an RFC/FCP).

* The ![Funded][] badge indicates that the owner has committed and work will be funded by their employer or other sources.
* The ![Team][] badge indicates a requirement where Team support is needed.

| Subgoal                                              | Owner(s) or team(s)                 | Status            |
| ---------------------------------------------------- | ----------------------------------- | ----------------- |
| overall program management                           | [nikomatsakis][], [Josh-Triplett][] | ![Funded][]       |
| arbitrary self types v2                              |                                     |                   |
| ↳ ~~author [RFC][RFC 3519]~~                         | ~~[Adrian Taylor][]~~               | ![Complete][]     |
| ↳ ~~approve [RFC][RFC 3519]~~                        | ~~[Lang]~~                          | ![Complete][]     |
| ↳ implementation                                     | [Adrian Taylor][]                   | ![Funded][]       |
| ↳ assigned reviewer                                  | ![Team] [Compiler]                  | ![Not approved][] |
| ↳ stabilization                                      | [Adrian Taylor][]                   | ![Funded][]       |
| derive smart pointer                                 |                                     |                   |
| ↳ ~~author [RFC][RFC 3621]~~                         | ~~[Alice Ryhl][]~~                  | ![Complete][]     |
| ↳ approve [RFC][RFC 3621]                            | ![Team][] [Lang]                    | ![Approved][]     |
| ↳ implementation                                     | [Xiang][]                           | ![Volunteer][]    |
| ↳ stabilization                                      | [Xiang][]                           | ![Volunteer][]    |
| `asm_goto`                                           |                                     |                   |
| ↳ ~~implementation~~                                 | -                                   | ![Complete][]     |
| ↳ real-world usage in Linux kernel                   | [Gary Guo]                          | ![Volunteer][]    |
| ↳ stabilization                                      | [Gary Guo]                          | ![Volunteer][]    |
| extended `offset_of` syntax                          |                                     |                   |
| ↳ stabilization                                      | ![Owner Needed][]                   | ![Volunteer][]    |
| RFL on Rust CI                                       |                                     |                   |
| ↳ implementation                                     | [Jakub Beránek][]                   | ![Funded][]       |
| ↳ policy draft                                       | [Jakub Beránek][]                   | ![Funded][]       |
| ↳ policy approval                                    | ![Team][] [Infra]                   | ![Not approved][] |
| Pointers to static in constants                      |                                     |                   |
| ↳ stabilization proposal                             | ![Help wanted][]                    |                   |
| ↳ stabilization decision                             | ![Team][] [Lang]                    | ![Not approved][] |
| Code-generation features and compiler options        |                                     |                   |
| ↳ ~~propose unstable `-Zfixed-x18` flag ([#748][])~~ | ~~[Alice Ryhl][]~~                  | ![Complete][]     |
| ↳ implement  `-Zfixed-x18` flag                      | [Xiang][]                           | ![Volunteer][]    |
| ↳ stabilization PR for `-Zfixed-x18`                 | [Xiang][]                           | ![Volunteer][]    |
| ↳ stabilization decision                             | ![Team][] [Compiler]                |                   |
| ↳ research and summarization for other flags         | ![Help wanted][]                    |                   |
| Custom builds of core/alloc                          |                                     |                   |
| ↳ stabilization proposal for `-Zbuild-std` mechanism | ![Help wanted][]                    |                   |
| ↳ stabilize `-Zbuild-std` mechanism                  | ![Team][] [Cargo]                   | ![Not approved][] |
| ↳ stabilization proposal for subsetting std          |                                     |                   |
| ↳ stabilize subset of std                            | ![Team][] [Libs-API]                | ![Not approved][] |

### Support needed from the project

* Lang team:
    * Prioritize RFC and any related design questions (e.g., the unresolved questions)

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

### What do I do with this space?

*This is a good place to elaborate on your reasoning above -- for example, why did you put the design axioms in the order that you did? It's also a good place to put the answers to any questions that come up during discussion. The expectation is that this FAQ section will grow as the goal is discussed and eventually should contain a complete summary of the points raised along the way.*

[Funded]: https://img.shields.io/badge/Funded-yellow
[Volunteer]: https://img.shields.io/badge/Volunteer-yellow
[Not funded]: https://img.shields.io/badge/Not%20yet%20funded-red
[Approved]: https://img.shields.io/badge/Approved-green
[Not approved]: https://img.shields.io/badge/Not%20yet%20approved-red
[Complete]: https://img.shields.io/badge/Complete-green
[TBD]: https://img.shields.io/badge/TBD-red
[Team]: https://img.shields.io/badge/Team%20ask-red
[Gary Guo]: https://github.com/nbdd0121
[Owner Needed]: https://img.shields.io/badge/Owned%20Needed-blue
[Help wanted]: https://img.shields.io/badge/Help%20wanted-blue
[#748]: https://github.com/rust-lang/compiler-team/issues/748
[Lang]: https://www.rust-lang.org/governance/teams/lang
[Compiler]: https://www.rust-lang.org/governance/teams/infra
[Infra]: https://www.rust-lang.org/governance/teams/infra
[Libs-API]: https://www.rust-lang.org/governance/teams/library#team-libs-api
[Cargo]: https://www.rust-lang.org/governance/teams/cargo
[LangInfra]: https://www.rust-lang.org/governance/teams/infra
[Alice Ryhl]: https://github.com/Darksonn/
[Adrian Taylor]: https://github.com/adetaylor
[Xiang]: https://github.com/dingxiangfei2009
[Jakub Beránek]: https://github.com/Kobzol
[nikomatsakis]: https://github.com/nikomatsakis/
[Josh-Triplett]: https://github.com/Josh-Triplett/


