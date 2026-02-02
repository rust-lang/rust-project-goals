# Const Generics

| Metadata         |                                    |
| :--------------- | ---------------------------------- |
| Point of contact | @BoxyUwU                           |
| Status           | Proposed                           |
| Flagship         | Constify all the things            |
| Tracking issue   | [rust-lang/rust-project-goals#100] |
| Zulip channel    | N/A                                |
| Stabilization    | true                               |
| [lang] champion  | @nikomatsakis                      |

## Summary

Extend const generics in two independent directions, both aiming for stabilization:

* **`adt_const_params`**: Allow structs and enums as const generic arguments, not just integers.
* **`min_generic_const_args`**: Allow associated constants as const generic arguments (e.g., `Foo<T::ASSOC_CONST>`).

We will also model const generics in `a-mir-formality` and experiment with upstreaming those changes into the Rust specification.
This work also serves as a forcing function for advancing a-mir-formality and its integration into the Rust specification.

## Motivation

### The status quo

The `min_const_generics` feature is stable, but with significant limitations on what can be used as a const generic argument:

* **Only integers**: Const generic parameters are limited to integer types. You cannot use structs or enums, even simple ones like `struct Dimensions { width: u32, height: u32 }`.
* **Only literals or generic parameters**: You can write `Foo<5>` or `Foo<N>` (where `N` is a const generic parameter), but you cannot write `Foo<T::ASSOC_CONST>` to use an associated constant.

When using const generics it is common to run into these limitations and be unable to move forwards, having to rewrite your code to use workarounds or not use const generics at all. This is a poor user experience and makes the language feel incomplete.

### The next few steps

We are extending const generics in two independent directions:

**`adt_const_params`**: Extending const generic arguments to include structs and enums. The implementation is largely complete, but we need to:

* Publish an RFC defining which ADTs are permitted. Some structs may be excluded due to concerns about privacy and unsafe invariants when the compiler infers const values. The RFC will nail down the precise rules.
* Model the feature in a-mir-formality to ensure we have a solid specification.

**`min_generic_const_args` (MGCA)**: Extending const generic arguments to include associated constants. This is currently in a "full prototype" state (`feature(min_generic_const_args)` has been merged) with more work needed before stabilization.

### The "shiny future" we are working towards

Our ultimate goal is to stabilize all parts of the const generics feature that were left out of the minimum stabilization. Users should not encounter "functionality cliffs" where const generics suddenly stops working as well as type generics, forcing code to be rewritten to work around language limitations.

### Design axioms

*TBD*

### Work items over the next year

| Task                                              | Owner(s) | Notes |
|---------------------------------------------------|----------|-------|
| Publish and merge `adt_const_params` RFC          | @BoxyUwU |       |
| Model `adt_const_params` in a-mir-formality       | @BoxyUwU | @nikomatsakis to help |
| Stabilize `adt_const_params`                      |          |       |
| Finish `min_generic_const_args` implementation    | @BoxyUwU | Currently in "full prototype" state |
| Model `min_generic_const_args` in a-mir-formality | @BoxyUwU | @nikomatsakis to help |
| Stabilize `min_generic_const_args`                |          |       |

## Team asks

| Team       | Support level | Notes                                      |
|------------|---------------|--------------------------------------------|
| [lang]     | Large         | Stabilization decisions, directional alignment             |
| [compiler] | Small         | Code reviews                               |
| [types]    | Medium        | a-mir-formality modeling, design alignment |

## Frequently asked questions

### What is the role of lang vs types team in the stabilizations?

The question of what equality means and what kinds of ADTs (structs, enums) can be used as const values, intersects both lang and types (`adt_const_params`).

Design and stabilization of `min_generic_const_args` is purely a types team affair.