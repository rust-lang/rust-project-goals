# Resolve the biggest blockers to Linux building on stable Rust

| Metadata    |                                |
| ----------- | ------------------------------ |
| Short title | Rust-for-Linux                 |
| Owner(s)    | @nikomatsakis, @joshtriplett   |
| Teams       | [lang], [libs-api], [compiler] |
| Status      | Flagship                       |

## Summary

Stabilize unstable features required by Rust for Linux project including

* Stable support for RFL's customized ARC type
* Labeled goto in inline assembler and extended `offset_of!` support
* RFL on Rust CI
* Pointers to statics in constants

## Motivation

The [experimental support for Rust development in the Linux kernel][RFL.com] is a watershed moment for Rust, demonstrating to the world that Rust is indeed capable of targeting all manner of low-level systems applications. And yet today that support rests on a [number of unstable features][RFL#2], blocking the effort from ever going beyond experimental status. For 2024H2 we will work to close the largest gaps that block support.
 
[RFL.com]: https://rust-for-linux.com/
[RFL#2]: https://github.com/Rust-for-Linux/linux/issues/2

### The status quo

The [Rust For Linux (RFL)][RFL] project has been accepted into the Linux kernel in experimental status. The project's goal, as described in the [Kernel RFC introducing it](https://lore.kernel.org/lkml/20210414184604.23473-1-ojeda@kernel.org/), is to add support for authoring kernel components (modules, subsystems) using Rust. Rust would join C as the only two languages permitted in the linux kernel. This is a very exciting milestone for Rust, but it's also a big challenge.

Integrating Rust into the Linux kernel means that Rust must be able to interoperate with the kernel's low-level C primitives for things like locking, linked lists, allocation, and so forth.
This interop requires Rust to expose low-level capabilities that don't currently have stable interfaces.

[RFL]: https://rust-for-linux.com/
[pinned-init]: https://rust-for-linux.com/pinned-init
[arclk]: https://rust-for-linux.com/arc-in-the-linux-kernel

The dependency on unstable features is the biggest blocker to Rust exiting "experimental" status. Because unstable features have no kind of reliability guarantee, this in turn means that RFL can only be built with a specific, pinned version of the Rust compiler. This is a challenge for distributions which wish to be able to build a range of kernel sources with the same compiler, rather than having to select a particular toolchain for a particular kernel version.

Longer term, having Rust in the Linux kernel is an opportunity to expose more C developers to the benefits of using Rust. But that exposure can go both ways. If Rust is constantly causing pain related to toolchain instability, or if Rust isn't able to interact gracefully with the kernel's data structures, kernel developers may have a bad first impression that causes them to write off Rust altogether. We wish to avoid that outcome. And besides, the Linux kernel is exactly the sort of low-level systems application we want Rust to be great for!

For deeper background, please refer to these materials:

* The article on the latest Maintainer Summit: [Committing to Rust for kernel code](https://lwn.net/Articles/952029/)
* The [LWN index on articles related to Rust in the kernel](https://lwn.net/Kernel/Index/#Development_tools-Rust)
* [The latest status update at LPC](https://www.youtube.com/watch?v=qvlgIaYrd3g).
* [Linus talking about Rust](https://www.youtube.com/watch?v=OvuEYtkOH88&t=335s).
* [Rust in the linux kernel, by Alice Ryhl](https://www.youtube.com/watch?v=CEznkXjYFb4)
* [Using Rust in the binder driver, by Alice Ryhl](https://www.youtube.com/watch?v=Kt3hpvMZv8o)

### The next six months

The RFL project has a [tracking issue][rfl2] listing the unstable features that they rely upon. After discussion with the RFL team, we identified the following subgoals as the ones most urgent to address in 2024. Closing these issues gets us within striking distance of being able to build the RFL codebase on stable Rust.

* Stable support for RFL's customized ARC type
* Labeled goto in inline assembler and extended `offset_of!` support
* RFL on Rust CI ([done now!])
* Pointers to statics in constants

#### Stable support for RFL's customized ARC type

One of Rust's great features is that it doesn't "bake in" the set of pointer types.
The common types users use every day, such as `Box`, `Rc`, and `Arc`, are all (in principle) library defined.
But in reality those types enjoy access to some unstable features that let them be used more widely and ergonomically.
Since few users wish to define their own smart pointer types, this is rarely an issue and there has been relative little pressure to stabilize those mechanisms.

The RFL project needs to integrate with the Kernel's existing reference counting types and intrusive linked lists.
To achieve these goals they've created their own variant of [`Arc`][arclk] (hereafter denoted as `rfl::Arc`),
but this type cannot be used as idiomatically as the `Arc` type found in `libstd` without two features:

* The ability to be used in methods (e.g., `self: rfl::Arc<Self>`), aka "arbitrary self types", specified in [RFC #3519].
* The ability to be coerce to dyn types like `rfl::Arc<dyn Trait>` and then support invoking methods on `Trait` through dynamic dispatch.
    * This requires the use of two unstable traits, `CoerceUnsized` and `DynDispatch`, neither of which are close to stabilization.
    * However, [RFC #3621] provides for a "shortcut" -- a stable interface using `derive` that expands to those traits, leaving room to evolve the underlying details.

Our goal for 2024 is to close those gaps, most likely by implementing and stabilizing [RFC #3519] and [RFC #3621].

[rfl2]: https://github.com/Rust-for-Linux/linux/issues/2

#### Labeled goto in inline assembler and extended `offset_of!` support

These are two smaller extensions required by the Rust-for-Linux kernel support.
Both have been implemented but more experience and/or development may be needed before stabilization is accepted.

#### RFL on Rust CI

> **Update**: Basic work was completed in [PR #125209] by Jakub Beránek during the planning process!
> We are however still including a team ask of T-compiler to make sure we have agreed around the policy
> regarding breakage due to unstable features.

[PR #125209]: https://github.com/rust-lang/rust/pull/125209

Rust sometimes integrates external projects of particular importance or interest into its CI.
This gives us early notice when changes to the compiler or stdlib impact that project.
Some of that breakage is accidental, and CI integration ensures we can fix it without the project ever being impacted.
Otherwise the breakage is intentional, and this gives us an early way to notify the project so they can get ahead of it.

Because of the potential to slow velocity and incur extra work,
the bar for being integrated into CI is high, but we believe that Rust For Linux meets that bar.
Given that RFL would not be the first such project to be integrated into CI,
part of pursuing this goal should be establishing clearer policies on when and how we integrate external projects into our CI,
as we now have enough examples to generalize somewhat.

#### Pointers to statics in constants

The RFL project has a need to create vtables in read-only memory (unique address not required). The current implementation relies on the `const_mut_refs` and `const_refs_to_static` features ([representative example](https://godbolt.org/z/r58jP6YM4)). Discussion has identified some questions that need to be resolved but no major blockers.

### The "shiny future" we are working towards

The ultimate goal is to enable smooth and ergonomic interop between Rust and the Linux kernel's idiomatic data structures.

In addition to the work listed above, there are a few other obvious items that the Rust For Linux project needs. If we can find owners for these this year, we could even get them done as a "stretch goal":

#### Stable sanitizer support

Support for building and using sanitizers, in particular KASAN.

#### Custom builds of core/alloc with specialized configuration options

The RFL project builds the stdlib with a number of configuration options to eliminate undesired aspects of libcore (listed in [RFL#2][]). They need a standard way to build a custom version of core as well as agreement on the options that the kernel will continue using.

#### Code-generation features and compiler options

The RFL project requires various code-generation options. Some of these are related to custom features of the kernel, such as X18 support ([rust-lang/compiler-team#748]) but others are codegen options like sanitizers and the like. Some subset of the options listed on [RFL#2][] will need to be stabilized to support being built with all required configurations, but working out the precise set will require more effort.

#### Ergonomic improvements

Looking further afield, possible future work includes more ergonomic versions of the [special patterns for safe pinned initialization](https://rust-for-linux.com/the-safe-pinned-initialization-problem) or a solution to [custom field projection for pinned types or other smart pointers](https://github.com/rust-lang/rfcs/pull/3318).

## Design axioms

* **First, do no harm.** If we want to make a good first impression on kernel developers, the minimum we can do is fit comfortably within their existing workflows so that people not using Rust don't have to do extra work to support it. So long as Linux relies on unstable features, users will have to ensure they have the correct version of Rust installed, which means imposing labor on all Kernel developers.
* **Don't let perfect be the enemy of good.** The primary goal is to offer stable support for the particular use cases that the Linux kernel requires. Wherever possible we aim to stabilize features completely, but if necessary, we can try to stabilize a subset of functionality that meets the kernel developers' needs while leaving other aspects unstable.

## Ownership and team asks

Here is a detailed list of the work to be done and who is expected to do it. This table includes the work to be done by owners and the work to be done by Rust teams (subject to approval by the team in an RFC/FCP).

* The ![Team][] badge indicates a requirement where Team support is needed.

| Subgoal                            | Owner(s) or team(s)          | Notes                     |
| ---------------------------------- | ---------------------------- | ------------------------- |
| Overall program management         | @nikomatsakis, @joshtriplett |                           |
| Arbitrary self types v2            | @adetaylor                   |                           |
| ↳ ~~author RFC~~                   |                              | ![Complete][] [RFC #3519] |
| ↳ ~~RFC decision~~                 | ~~[lang]~~                   | ![Complete][]             |
| ↳ Implementation                   |                              |                           |
| ↳ Standard reviews                 | ![Team] [compiler]           |                           |
| ↳ Stabilization decision           | ![Team] [lang]               |                           |
| Derive smart pointer               | @Darksonn                    |                           |
| ↳ ~~author RFC~~                   |                              | [RFC #3621]               |
| ↳ RFC decision                     | ![Team][] [lang]             | ![Complete][]             |
| ↳ Implementation                   | @dingxiangfei2009            |                           |
| ↳ Author stabilization report      | @dingxiangfei2009            |                           |
| ↳ Stabilization decision           | ![Team][] [lang]             |                           |
| `asm_goto`                         | @nbdd0121                    |                           |
| ↳ ~~implementation~~               |                              | ![Complete][]             |
| ↳ Real-world usage in Linux kernel |                              |                           |
| ↳ Extend to cover full RFC         |                              |                           |
| ↳ Author stabilization report      |                              |                           |
| ↳ Stabilization decision           | ![Team][] [lang]             |                           |
| Extended `offset_of` syntax        | @dingxiangfei2009            |                           |
| ↳ Stabilization report             |                              |                           |
| ↳ Stabilization decision           | ![Team][] [libs-api]         |                           |
| ~~RFL on Rust CI~~                 | @Kobzol                      |                           |
| ↳ ~~implementation~~               |                              | ![Complete][] [#125209]   |
| ↳ Policy draft                     |                              |                           |
| ↳ Policy decision                  | ![Team][] [compiler]         |                           |
| Pointers to static in constants    | @nikomatsakis                |                           |
| ↳ Stabilization report             |                              |                           |
| ↳ Stabilization decision           | ![Team][] [lang]             |                           |

### Support needed from the project

* Lang team:
    * Prioritize RFC and any related design questions (e.g., the unresolved questions)

## Outputs and milestones

### Outputs

*Final outputs that will be produced*

### Milestones

*Milestones you will reach along the way*

## Frequently asked questions

None yet.



