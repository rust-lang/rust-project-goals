# Reborrow traits

| Metadata         |                                    |
| :--------------- | ---------------------------------- |
| Point of contact | @aapoalas                          |
| Status           | Proposed                           |
| Flagship         | Beyond the `&`                     |
| Tracking issue   | [rust-lang/rust-project-goals#399] |
| Zulip channel    | [#t-lang/custom-refs][channel]     |
| [lang] champion  | @tmandry                           |

[channel]: https://rust-lang.zulipchat.com/#narrow/channel/522311-t-lang.2Fcustom-refs

## Summary

Bring the `Reborrow` and `CoerceShared` trait to nightly Rust, enabling iteration based on user feedback. Begin work on nontrivial cases where more than one lifetime is being reborrowed or coerced into shared

## Motivation

Reborrowing is fundamental to Rust's ergonomics with references. When you pass a `&mut T` to a function, Rust automatically reborrows it so you can use it again after the call. But this doesn't work for user-defined types that wrap references, like `Option<&mut T>`, `Pin<&mut T>`, or custom smart pointers.

Today, users work around this with `.as_deref_mut()` or `.reborrow()` methods, but these have a critical limitation: values derived from a reborrow cannot be returned from the function that called `.reborrow()`. The lifetime gets constrained to the reborrow call site. True reborrowing doesn't have this constraint.

### The status quo

The 2025H2 period produced a working implementation of the `Reborrow` and `CoerceShared` traits for types with a single lifetime parameter and trivial memory layouts. The exact mechanism for expanding the implementation from trivial cases to non-trivial memory layouts and sets of reborrowed lifetimes remains an open problem.

### What we propose to do about it

This year we continue iteration on the traits based on user feedback, focusing on:

1. **Gather feedback from users** - Especially from `reborrow` crate users and the Rust for Linux project
1. **Overcome known limitations** - Support types with multiple lifetime parameters where one or more of them gets reborrowed.
1. **Support non-trivial `CoerceShared`** - Support `CoerceShared` operations that reorder or drop fields.
1. **Expand safety and validity checks** - `CoerceShared` performs an implicit type transmute and must be extensively tested and validated.
1. **Prepare for RFC** - Refine the [draft RFC](https://github.com/aapoalas/rfcs/blob/autoreborrow-traits/text/0000-autoreborrow-traits.md) based on implementation experience.

The fundamental design philosophy remains:
- Reborrowing is "a memory copy with added lifetime analysis" - no user code is run.
- The traits must achieve *true* reborrowing where derived values can be returned from the function.
- Performance cost must be trivial as reborrowing is performed at every coercion site.
- Prevent abuse - `CoerceShared` must not slide into the realm of a generic `Coerce` trait.

### Work items over the next year

| Task                                          | Owner(s)  | Notes |
| --------------------------------------------- | --------- | ----- |
| Land first implementation PR to nightly       | @aapoalas |       |
| Solicit wide feedback on the feature          | @aapoalas |       |
| Continue experiment based on experience       | @aapoalas |       |

## Team asks

| Team       | Support level | Notes                                         |
| ---------- | ------------- | --------------------------------------------- |
| [lang]     | Medium        | Continued experiment support, design feedback |
| [compiler] | Small         | Standard reviews for trait implementation PRs |

## Frequently asked questions

### How does this relate to Pin ergonomics?

Pin currently has special-cased reborrow support in the compiler. Once reborrow traits are stable, `Pin<&mut T>` can be reimplemented using the general mechanism, removing the special case. The Pin ergonomics goal is proceeding in parallel.

### Why the shift from associated type to type parameter for CoerceShared?

Originally `CoerceShared` had `type Target` to prevent multiple coercion targets (which could enable abuse as a general coercion trait). But the trait method that motivated this fear was found unnecessary, so allowing multiple targets via `trait CoerceShared<Target: Copy> {}` is now safe and enables more flexibility - like a collection of `&mut`s all coercing to `&`s.

### Why limit to single lifetime initially?

Supporting multiple lifetimes requires dealing with rmeta (Rust metadata) complexity. Focusing on single-lifetime reborrowing first lets us get the core functionality working before tackling that additional complexity.

[rust-lang/rust-project-goals#399]: https://github.com/rust-lang/rust-project-goals/issues/399
[lang]: https://github.com/rust-lang/lang-team
[compiler]: https://github.com/rust-lang/compiler-team
