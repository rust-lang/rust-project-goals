# Rust for Linux

| Metadata         |                        |
|:-----------------|------------------------|
| Short title      | Rust for Linux         |
| What and why     | Build the Linux kernel with only stable language features. |
| Point of contact | @tomassedovic          |
| Application area | Systems & embedded     |

## Summary

Rust is now an integral part of the Linux kernel. There are mainline drivers built in it, tooling and it's [even directly in the kernel itself][drm-panic].

The [Rust for Linux project][rfl] is leading the development and the Rust Project is helping to facilitate some of the coordination.

[rfl]: https://rust-for-linux.com/
[drm-panic]: https://rust-for-linux.com/drm-panic-qr-code-generator

While the project is built on a stable version of the Rust compiler, it relies on language features that are unstable (via the `#![feature(...)]` declarations). Ultimately, we want to stabilize all the features Rust for Linux relies on and support any platform that Linux itself supports.

## Motivation

### The status quo

The [Rust for Linux page][rfl] has a great overview of what's supported and being worked on at the moment.

The project has to use the feature-gated functionality and it's currently limited to platforms that LLVM supports.

The Rust Project has a regular meeting with the Rust for Linux folks every two weeks and that is where we synchronise on status and discuss any potential road blocks and issues or topics that came out.

We have representatives from the Language and Compiler teams present.

We have also been tracking the coordination through Project goals and this is a continuation of that effort.

### What we are shooting for

* Build Linux kernel releases using the stable Rust language (no feature gates).
* Support all the targets that Linux supports.
* Address the [long tail of features Rust for Linux needs](https://github.com/rust-for-linux/linux/issues/354)

### Key use cases

* **Reduce the risk of critical errors:** By compartmentalising the unsafe parts of the code base and having a clear boundary, we can build code that will have fewer bugs. We can rewrite notoriously tricky sections and provide better primitives.

* **Improve code quality across the kernel:** In addition to the direct benefits of the Rust code, the Kernel is seeing improvements in the *C* side as well. We are seeing improvements of existing APIs in terms of `const`-ness, clarity and safety. Even people working solely with C have started pushing for better API documentation. Having Rust with the constantly maintained high bar that the Rust for Linux folks started results in a *better codebase overall*.

* **Bring in new contributors:** It is a frequent theme within the Rust community that people who would never felt confident writing C dive into low-level topics and able to learn, experiment and *ship* code that is safe to be integrated into larger systems.

### Design axioms

**Don't let perfect be the enemy of good**™: The primary goal is to offer stable support for the particular use cases that the Linux kernel requires. Wherever possible we aim to stabilize features completely, but if necessary, we can try to stabilize a subset of the functionality that meets the kernel developers' needs while leaving other aspects unstable.


## 2026 Goals

(((ROADMAP GOALS: Rust for Linux)))

`compiler_builtins` TODO(tomassedovic) the previous issue also lists compiler builtins. What are those?


## Other topics

These are other topics Rust for Linux is interested in but that do not have a project goal. There are more (please see the linked lists below), but for 2026 we considered these to be some we would like to focus on. In order of priority (roughly):

  - `rustfmt`: removing the `//` hack inside imports ("kernel vertical style").
    + Essentially being able to trigger the logic that currently gets triggered when `rustfmt` detects a comment.

  - Coherence domains.
    + Not just for the orphan rule, but also to e.g. define inherent methods on a dependency like `core` and `std` do to each other.
    + We will be splitting soon our `kernel` crate in smaller crates.

  - Niche optimizations.
    + At least for Rust `enum`s that contain pointers to aligned objects (e.g. 4 byte alignment in XArray, or to a page, etc.).

  - Clippy: support for more tagged comments.
    + `// PANIC: ...`, `// CAST: ...` and `// ORDERING: ...`.

  - `extern` types.
    + Currently we use `Opaque` (a `UnsafeCell<MaybeUninit<T>>` + `PhantomPinned` for avoiding uniqueness for mutable references).

  - `rustdoc`: Stabilizing `--output-format=doctest`.

  - `-Coverflow-checks=report`.
    + i.e. a new mode that allows to "report and continue" (wrapping), like UBSan and similar tools.

  - CFI related improvements.
    + `fn_cast!` macro and `bindgen` improvements to generate the annotation for CFI encoding on structs.

  - Support packed types to be aligned or to contain aligned members.
    + And `bindgen` support for it.

  - Custom prelude.

  - `build_assert!`.
    + Especially ways to fail earlier and/or improve the error messages, similar to [GCC's](https://gcc.gnu.org/onlinedocs/gcc/Common-Function-Attributes.html#index-error-function-attribute) and [Clang's](https://clang.llvm.org/docs/AttributeReference.html#error-warning) `error`/`warning` attribute.

  - Specialization for improving the performance of our `alloc` types.

  - `--emit=noreturn`.
    + We may be able to improve `objtool`, but having this would significantly improve the situation.

  - Safe transumations.
    + We may vendor `zerocopy` for now, but long-term we are interested in seeing parts of it make it into the standard library.

  - Contracts/better safety comment tools.

Please see more details at the following Rust for Linux lists:

  - [Rust unstable features needed for the kernel](https://github.com/Rust-for-Linux/linux/issues/2).

  - [Rust wanted features](https://github.com/Rust-for-Linux/linux/issues/354).

  - [`core` wanted features & bugfixes](https://github.com/Rust-for-Linux/linux/issues/514).

  - [`rustc`](https://github.com/Rust-for-Linux/linux/issues/355).

  - [`rustdoc` wanted features & bugfixes ](https://github.com/Rust-for-Linux/linux/issues/350).

  - [`rustfmt` wanted features & bugfixes](https://github.com/Rust-for-Linux/linux/issues/398).

  - [Clippy wanted features & bugfixes](https://github.com/Rust-for-Linux/linux/issues/349).


## Frequently asked questions

TODO

* when is this done? (A: when the work here is all stable?)
* how do the goals relate to each other (note the interdependencies)
