# Full Const Generics

| Metadata         |                                     |
| :--------------- | ----------------------------------- |
| Point of contact | @BoxyUwU                            |
| Status           | Proposed                            |
| What and why     | Finish "the rest" of Const Generics |
| Roadmap          | Constify all the things             |
| Roadmap          | Rust for Linux                      |
| Tracking issue   | [rust-lang/rust-project-goals#100]  |
| Zulip channel    | [#project-const-generics]           |
| Highlight        | Language changes                    |
| [lang] champion  | @nikomatsakis                       |
| [types] champion | @BoxyUwU                            |

[zulip]: https://rust-lang.zulipchat.com/#narrow/channel/260443-project-const-generics

## Summary

"Finish" Const Generics, turning the current minimum stabilization of Const Generics into a fully featured cohesive part of the language.

## Motivation

### The status quo

A minimum form of Const Generics is stable, but with significant limitations on what can be done. There are two main restrictions:

* **Limited Types of Const Generic Parameters**: Only integer types, char and bool are allowed as the types of const generics. Structs and enums are disallowed, even simple ones such as `struct Dimension { width: u32, height: u32}`.
* **Limited Arguments to Const Generic Parameters**: Only `_`, `N` (referring to a generic parameter N), and expressions which can be immediately evaluated, are supported as an argument to const generics. You cannot write `Foo<T::ASSOC>` to use an associated constant.

When using const generics it is common to run into these limitations and be unable to move forwards, having to rewrite your code to use workarounds or not use const generics at all. This is a poor user experience and makes the language feel incomplete.

### The next few steps

We are extending const generics in two independent directions to resolve both of these limitations.

**ADT Const Parameters**: Extending const generic parameters to include structs and enums.

The implementation for this is under `feature(adt_const_params)` and is largely complete. There are a few next steps here:

* Split out a `min_adt_const_params` feature which does not support structs with private fields
* Publish an RFC defining which ADTs are permitted. Some structs may be excluded due to concerns about privacy and unsafe invariants when the compiler infers const values. The RFC will nail down the precise rules.

**Generic Const Arguments**: Extending const generic arguments to support some kinds of expressions which cannot be immediately evaluated (for example `T::ASSOC_CONST`).

`feature(min_generic_const_args)` currently exists as a prototype of a potential *minimal* stabilizeable improvement. We intend to make more progress on the implementation for this feature so that it is more ready for stabilization.

We also intend to work on a more "full" extension than the above feature which will have less restrictions. We have an idea of what this should look like but it is has yet to be prototyped.

**Formalization**: Modelling const generics in `a-mir-formality`.

We will also model const generics, including unstable features, in `a-mir-formality` and experiment with upstreaming those changes into the unstable reference.
This work should help with stabilization of the above features by making it easier to communicate the behaviour of the features.

This work also serves as a forcing function for advancing a-mir-formality and its integration into the Rust specification.

### The "shiny future" we are working towards

Our ultimate goal is to stabilize all parts of the const generics feature that were left out of the minimum stabilization. Users should not encounter "functionality cliffs" where const generics suddenly stops working as well as type generics, forcing code to be rewritten to work around language limitations.

### Work items over the next year

| Task                                              | Owner(s) | Notes |
|---------------------------------------------------|----------|-------|
| Publish and merge `adt_const_params` RFC          | @BoxyUwU |       |
| Model `adt_const_params` in a-mir-formality       | @BoxyUwU | @nikomatsakis to help |
| Stabilize `adt_const_params`                      | @BoxyUwU |       |
| Finish `min_generic_const_args` implementation    | @BoxyUwU | Currently is a working prototype |
| Model `min_generic_const_args` in a-mir-formality | @BoxyUwU | @nikomatsakis to help |
| Prototype a "full" generic const args             | @BoxyUwU |       |
| Model the "full" generic const args prototype in a-mir-formality | @BoxyUwU | @nikomatsakis to help |

## Team asks

| Team       | Support level | Notes                                          |
|------------|---------------|----------------------------------------------- |
| [lang]     | Large         | Stabilization decisions, directional alignment |
| [types]    | Large         | a-mir-formality modeling, design alignment, reviews    |

## Frequently asked questions

### What is the role of lang vs types team in the stabilizations?

The questions of what equality means and what kinds of ADTs (structs, enums) can be used as const values, intersects both lang and types (`adt_const_params`).