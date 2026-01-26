# Const Generics

| Metadata         |                                    |
| :--------------- | ---------------------------------- |
| Point of contact | @BoxyUwU                           |
| Status           | Proposed                           |
| Tracking issue   | [rust-lang/rust-project-goals#100] |
| Zulip channel    | N/A                                |
| [lang] champion  | @nikomatsakis                      |

## Summary

Work towards stabilizing the remaining const generics functionality that was left out of the original `min_const_generics` feature.

## Motivation

### The status quo

The `min_const_generics` feature is stable, but with a number of limitations.

When using const generics it is common to run into these limitations and be unable to move forwards, having to rewrite your code to use workarounds or not use const generics at all. This is a poor user experience and makes the language feel incomplete.

Our ultimate goal is to stabilize all parts of the const generics feature that were left out of the minimum stabilization. For the users to not encounter "functionality cliffs" where const generics
suddenly stops working as well as type generics, forcing code to be rewritten to work around language limitations.


### What we propose to do about it

`feature(min_generic_const_args)` has been merged and is now in a "full prototype" state, with a lot of work still left to do before it can be stabilized.

We have a Zulip channel for `feature(adt_const_params)` (Const parameters with arbitrary user-defined types): [#project-const-generics/adt_const_params-rfc](https://rust-lang.zulipchat.com/#narrow/channel/551659-project-const-generics.2Fadt_const_params-rfc). We need to open the RFC.

We still need to do: `feature(generic_arg_infer)` and `feature(associated_const_equality)` as well.


### Work items over the next year

| Task                                              | Owner(s) | Notes |
|---------------------------------------------------|----------|-------|
| Finish up `feature(min_generic_const_args)`       |          |       |
| Publish and merge `feature(adt_const_params)` RFC |          |       |
| Implement `feature(generic_arg_infer)`            |          |       |
| Implement `feature(associated_const_equality)`    |          |       |
| Stabilization?                                    |          |       |


## Team asks

| Team       | Support level | Notes                    |
|------------|---------------|--------------------------|
| [lang]     | Medium        | Code reviews, RFC review |
| [compiler] | Medium        | Reviews                  |
