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


## Frequently asked questions

TODO

* when is this done? (A: when the work here is all stable?)
* how do the goals relate to each other (note the interdependencies)
