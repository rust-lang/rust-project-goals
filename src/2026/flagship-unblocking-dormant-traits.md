# Unblocking dormant traits

## Summary

Long-blocked type system improvements finally ship, enabling extern types, scalable vectors, and painless trait refactoring.

## Motivation

### The status quo

Rust's trait system is one of its most powerful features, but a number of long-desired improvements have been blocked on foundational work in the trait solver—the compiler component responsible for proving trait bounds, normalizing associated types, and more.

The current trait solver has accumulated technical debt and limitations over the years. This has led to:

* **Known soundness bugs** that can't be fixed without breaking the solver's assumptions ([tracking board][unsoundnesses])
* **Blocked language features** like coinductive trait semantics, perfect derive, and better handling of higher-ranked types
* **An inability to express new concepts** in the type system, such as a richer `Sized` hierarchy needed for extern types or scalable vectors

[unsoundnesses]: https://github.com/orgs/rust-lang/projects/61/views/1

The [next-generation trait solver](./next-solver.md) has been in development since late 2022 and is already stable for coherence checking (since Rust 1.84). Stabilizing it everywhere will fix these soundness issues, unblock the stalled features, and provide a foundation for future type system work.

With that foundation in place, we can:

* **Extend the `Sized` hierarchy** ([RFC #3729], [RFC #3838]) to express types that are neither `Sized` nor `?Sized`—like extern types ([RFC #1861], no size at all) or scalable vectors (size known at runtime but not compile time)
* **Enable backward-compatible trait refactoring** so library authors can split traits into smaller pieces without breaking downstream code

[RFC #1861]: https://rust-lang.github.io/rfcs/1861-extern-types.html
[RFC #3729]: https://rust-lang.github.io/rfcs/3729-sized-hierarchy.html
[RFC #3838]: https://github.com/rust-lang/rfcs/pull/3838

### What we are shooting for

By the end of 2026:

* **New trait solver on stable.** The next-generation trait solver replaces the old implementation everywhere, fixing known soundness bugs and providing a foundation for future type system work.
* **Extern types work.** You can declare `extern type Foo;` and use it in FFI code without the compiler demanding a size it can't know.
* **Trait evolution is field-testing on nightly.** Library authors can experiment with splitting traits into smaller pieces without breaking downstream code, with the standard library running field trials.
* **Scalable vectors are experimenting on nightly.** Developers targeting AArch64 SVE can use scalable vector types that adapt to hardware capabilities.

### Key use cases

* **Soundness without surprises**: Known trait solver bugs are fixed. Code that compiles continues to compile, except where it was relying on unsound behavior.

* **C library bindings**: FFI wrappers for C libraries with opaque types (like `FILE *`) can express that these types exist but have no meaningful size in Rust.

* **Standard library evolution**: The standard library team can begin field-testing trait refactoring—extracting common functionality into supertraits or reorganizing hierarchies—with the supertrait auto-impl feature, paving the way for ecosystem-wide adoption.

* **High-performance SIMD**: Performance-critical code for AArch64 servers can experiment with scalable vector extensions that adapt to hardware vector width.

### Design axioms

* **Fix the foundations first.** Many desired features share common blockers in the trait solver. Rather than working around limitations repeatedly, we invest in fixing the underlying infrastructure once.

* **Unblock, then polish.** Ship the core capabilities that unblock use cases, even if ergonomic improvements come later.

* **Preserve the ecosystem.** Backward compatibility is paramount. Soundness fixes must not break working code except where that code relies on unsound behavior.

## 2026 goals

(((FLAGSHIP GOALS: Unblocking dormant traits)))

## Frequently asked questions

### How do these goals relate to each other?

The goals form a foundation-and-application pattern:

* **Next-generation trait solver** is the foundational layer. The new solver fixes soundness issues and provides the infrastructure needed for more sophisticated type system features.

* **Sized hierarchy and scalable vectors** extends the type system to express new categories of types, building on the sound foundation the new solver provides.

* **Supertrait item implementations** enables backward-compatible trait evolution—largely independent of the solver work, but benefits from landing after the new solver is stable.

### Will these changes break my code?

The goal is to avoid breakage. The next-generation trait solver has been extensively tested via crater runs, and known regressions are either fixed or were cases where the old solver was unsound. The Sized hierarchy is additive—existing code using `Sized` and `?Sized` continues to work. Supertrait item implementations is specifically designed to make changes that *would* have been breaking become non-breaking.
