# Reborrow traits

| Metadata         |                                    |
| :--------------- | ---------------------------------- |
| Point of contact | @aapoalas                          |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#399] |
| Zulip channel    | N/A                                |
| [lang] champion  | @tmandry                           |

## Summary

Continue iterating on the `Reborrow` and `CoerceShared` traits, moving from initial prototypes toward a complete nightly implementation with proper coherence checking and safety validation.

## Motivation

### The status quo

Reborrowing is fundamental to Rust's ergonomics with references. When you pass a `&mut T` to a function, Rust automatically reborrows it so you can use it again after the call. But this doesn't work for user-defined types that wrap references, like `Option<&mut T>`, `Pin<&mut T>`, or custom smart pointers.

Today, users work around this with `.as_deref_mut()` or `.reborrow()` methods, but these have a critical limitation: values derived from a reborrow cannot be returned from the function that called `.reborrow()`. The lifetime gets constrained to the reborrow call site. True reborrowing doesn't have this constraint.

The 2025H2 period made significant progress:
- The `Reborrow` trait is working for types with exclusive reference semantics (though currently requires `let mut` binding - a known bug)
- `CoerceShared` trait design has evolved: changed from associated type to `trait CoerceShared<Target: Copy> {}` to allow multiple coercion targets
- Coherence checking approach has been refined: only the first lifetime participates in reborrowing as a simplification
- Field mapping storage for `CoerceShared` remains an open problem

### What we propose to do about it

This year we continue experimental iteration on the traits, focusing on:

1. **Fix known issues with `Reborrow`** - Remove the spurious `let mut` requirement
2. **Complete `CoerceShared` implementation** - Solve the field mapping storage problem and get basic functionality working
3. **Add safety and validity checks** - Currently absent, these are required before broader experimentation
4. **Gather feedback from users** - Especially from `reborrow` crate users and the Rust for Linux project
5. **Prepare for RFC** - A [draft RFC](https://github.com/aapoalas/rfcs/blob/autoreborrow-traits/text/0000-autoreborrow-traits.md) exists; refine based on implementation experience

The fundamental design philosophy remains:
- Reborrowing is "a memory copy with added lifetime analysis" - no user code runs
- Must achieve *true* reborrowing where derived values can be returned past the reborrow point
- Performance must be trivial since reborrow is checked at every coercion site
- Prevent abuse as a general type coercion mechanism

### Work items over the next year

| Task                                          | Owner(s)  | Notes |
| --------------------------------------------- | --------- | ----- |
| Implementation and experimentation on nightly | @aapoalas |       |

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
