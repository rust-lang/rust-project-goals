# Stabilize tooling needed by Rust for Linux

| Metadata         |                                    |
|------------------|------------------------------------|
| Short title      | Rust-for-Linux                     |
| Point of contact | @nikomatsakis                      |
| Teams            | [compiler]                         |
| Status           | Proposed for flagship              |
| Tracking issue   | [rust-lang/rust-project-goals#116] |

## Summary

Continue working towards Rust for Linux on stable, turning focus from language features to compiler and tooling.

## Motivation

This goal continues our push to support the Linux kernel building on stable Rust. The focus in 2025H1 is shifting from language features, which were largely completed in 2024H2, towards compiler flags and tooling support. The Linux Kernel makes use of a number of unstable options in the compiler for target specific optimizations, code hardening, and sanitizer integration. It also requires a custom build of the standard library and has hacky integration with rustdoc to enable the use of doctests. We are looking to put all of these items onto a stable foundation.

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

### What we have done so far

We began the push towards stable support for RFL in 2024H2 with [a project goal focused on language features](https://github.com/rust-lang/rust-project-goals/issues/116). Over the course of those six months we:

* Stabilized the `CoercePointee` derive, supporting the kernel's use of smart pointers to model intrusive linked lists.
* Stabilized basic usage of `asm_goto`. Based on a survey of the kernel's usage, we [modified the existing design](https://github.com/rust-lang/rust/issues/132078) and also proposed [two](https://github.com/rust-lang/rust/issues/128464) [extensions](https://github.com/rust-lang/rust/pull/131523).
* Stabilized `offset_of` syntax applied to structs.
* Added Rust-for-Linux to the Rust CI to avoid accidental breakage.
* Stabilized support for pointers to static in constants. 

The one feature which was not stabilized yet is [arbitrary self types v2](https://github.com/rust-lang/rust/issues/44874), which reached "feature complete" status in its implementation. Stabilization is expected in early 2025.

We also began work on tooling stabilization with an [RFC proposing an approach to stabilizing ABI-modifying compiler flags](https://github.com/rust-lang/rfcs/pull/3716).

### The next six months

Over the next six months our goal is to stabilize the major bits of tooling used by the Rust for Linux project. Some of these work items are complex enough to be tracked independently as their own project goals, in which case they are linked.

* implementing RFC #3716 to stabilize ABI-modifying compiler flags to control code generation, sanitizer integration, and so forth:
    * arm64: `-Zbranch-protection`, `-Zfixed-x18`, `-Zuse-sync-unwind`.
    * x86: `-Zcf-protection`, `-Zfunction-return`, `-Zno-jump-tables`, `-Zpatchable-function-entry`, retpoline (`+retpoline-external-thunk,+retpoline-indirect-branches,+retpoline-indirect-calls`), SLS (`+harden-sls-ijmp,+harden-sls-ret`).
    * x86 32-bit: `-Zregparm=3`, `-Zreg-struct-return`.
    * LoongArch: `-Zdirect-access-external-data`.
    * production sanitizer flags: `-Zsanitizer=shadow-call-stack`, `-Zsanitizer=kcfi`, `-Zsanitizer-cfi-normalize-integer`.
* the ability to extract dependency info and to configure no-std without requiring it in the source file:
    * currently using `-Zbinary_dep_depinfo=y` and `-Zcrate-attr`
* stable rustdoc features allowing the RFL project to extract and customize rustdoc tests:
* clippy configuration (`.clippy.toml` in particular and `CLIPPY_CONF_DIR`);
* [a blessed way to rebuild std](./build-std.md): RFL needs a way to rebuild the standard library using stable calls to rustc. Currently building the standard library with rustc is not supported. This is a precursor to what is commonly called `-Zbuild-std`; it is also a blocker to making full use of API-modifying compiler flags and similar features, since they can't be used effectively unless the kernel is rebuilt.

In addition, as follow-up from 2024H2, we wish to complete [arbitrary self types v2][astv2] stabilization.

### The "shiny future" we are working towards

The ultimate target for this line of work is that Rust code in the Linux kernel builds on stable Rust with a Minimum Supported Rust Version (MSRV) tied to some external benchmark, such as Debian stable. This is the minimum requirement for Rust integration to proceed from an "experiment" so something that could become a permanent part of Linux.

Looking past the bare minimum, the next target would be making "quality of life" improvements that make it more ergonomic to write Rust code in the kernel (and similar codebases). One such example is the proposed experiment for [field projections](./field-projections.md).

## Design axioms

* **First, do no harm.** If we want to make a good first impression on kernel developers, the minimum we can do is fit comfortably within their existing workflows so that people not using Rust don't have to do extra work to support it. So long as Linux relies on unstable features, users will have to ensure they have the correct version of Rust installed, which means imposing labor on all Kernel developers.
* **Don't let perfect be the enemy of good.** The primary goal is to offer stable support for the particular use cases that the Linux kernel requires. Wherever possible we aim to stabilize features completely, but if necessary, we can try to stabilize a subset of functionality that meets the kernel developers' needs while leaving other aspects unstable.

## Ownership and team asks

Here is a detailed list of the work to be done and who is expected to do it. This table includes the work to be done by owners and the work to be done by Rust teams (subject to approval by the team in an RFC/FCP).

* The ![Team][] badge indicates a requirement where Team support is needed.

| Task                         | Owner(s) or team(s)                          | Notes |
|------------------------------|----------------------------------------------|-------|
| Discussion and moral support | ![Team][] [compiler][] [rustdoc][] [cargo][] |       |
| Overall program management   | @nikomatsakis                                |       |

### ABI-modifying compiler flags

Goal: stabilizing various ABI-modifying flags such as `-Zbranch-protection` and friends.

| Task                   | Owner(s) or team(s)    | Notes                                                   |
|------------------------|------------------------|---------------------------------------------------------|
| Author RFC             | @darksonn              | ![Completed][]                                          |
| RFC decision           | ![Team][] [compiler][] | RFC #3716, currently in PFCP                            |
| Implementation         | ![Help Wanted][]       | For each flag, need to move flags from `-Z` to `-C` etc |
| Standard reviews       | ![Team][] [compiler]   |                                                         |
| Stabilization decision | ![Team][] [compiler][] | For each of the relevant compiler flags                 |

### Extract dependency information, configure no-std externally

Goal: support extraction of dependency information (similar to `-Zbinary_dep_depinfo=y` today) and ability to write crates without explicit, per-crate `![no_std]` (achieved via `-Zcrate-attr` today).

Right now there is no plan for how to approach this. This task needs an owner to pick it up, make a plan, and execute.

| Task                   | Owner(s) or team(s)    | Notes |
|------------------------|------------------------|-------|
| Author a plan          | ![Help Wanted][]       |       |
| Implementation         | ![Help Wanted][]       |       |
| Standard reviews       | ![Team][] [compiler]   |       |
| Stabilization decision | ![Team][] [compiler][] |       |

### Rustdoc features to extract doc tests

Goal: stable rustdoc features sufficient to extract doc tests without hacky regular expressions

| Task                   | Owner(s) or team(s)   | Notes |
|------------------------|-----------------------|-------|
| Author RFC             | ![Help Wanted][]      |       |
| RFC decision           | ![Team][] [rustdoc][] |       |
| Implementation         | ![Help Wanted][]      |       |
| Standard reviews       | ![Team][] [rustdoc]   |       |
| Stabilization decision | ![Team][] [rustdoc][] |       |

### Clippy configuration

Goal: stabilized approach to customizing clippy (like `.clippy.toml` and `CLIPPY_CONF_DIR` today).

As discussed on [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/257328-clippy/topic/stablization.20of.20clippy.2Etoml.20a), the relevant policy is already correct, but documentation is needed.

| Task                   | Owner(s) or team(s)  | Notes |
|------------------------|----------------------|-------|
| Author documentation   | ![Help Wanted][]     |       |
| Stabilization decision | ![Team][] [clippy][] |       |

### Blessed way to rebuild std

See [build-std](./build-std.md) goal.
